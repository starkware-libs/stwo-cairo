// Constraints version: bc855610

use core::num::traits::Zero;
use stwo_constraint_framework::{
    LookupElementsImpl, PreprocessedColumn, PreprocessedColumnSet, PreprocessedColumnSetImpl,
    PreprocessedMaskValues, PreprocessedMaskValuesImpl,
};
use stwo_verifier_core::channel::{Channel, ChannelTrait};
use stwo_verifier_core::circle::{
    CirclePoint, CirclePointIndexTrait, CirclePointQM31AddCirclePointM31Trait,
};
use stwo_verifier_core::fields::Invertible;
use stwo_verifier_core::fields::m31::{M31, m31};
use stwo_verifier_core::fields::qm31::{QM31, QM31Impl, QM31Serde, QM31Zero, qm31_const};
use stwo_verifier_core::poly::circle::CanonicCosetImpl;
use stwo_verifier_core::utils::{ArrayImpl, pow2};
use stwo_verifier_core::{ColumnArray, ColumnSpan, TreeArray};
use crate::components::CairoComponent;
use crate::components::subroutines::mem_verify::mem_verify_evaluate;
use crate::components::subroutines::read_split::read_split_evaluate;
use crate::components::subroutines::verify_reduced_252::verify_reduced_252_evaluate;

pub const N_TRACE_COLUMNS: usize = 351;
pub const RELATION_USES_PER_ROW: [(felt252, u32); 5] = [
    ('RangeCheck_5_4', 2), ('MemoryAddressToId', 3), ('MemoryIdToBig', 3), ('RangeCheck_8', 4),
    ('PartialEcMul', 4),
];

#[derive(Drop, Serde, Copy)]
pub struct Claim {
    pub log_size: u32,
    pub pedersen_builtin_segment_start: u32,
}

#[generate_trait]
pub impl ClaimImpl of ClaimTrait {
    fn log_sizes(self: @Claim) -> TreeArray<Span<u32>> {
        let log_size = *(self.log_size);
        let preprocessed_log_sizes = array![log_size].span();
        let trace_log_sizes = ArrayImpl::new_repeated(N_TRACE_COLUMNS, log_size).span();
        let interaction_log_sizes = ArrayImpl::new_repeated(40, log_size).span();
        array![preprocessed_log_sizes, trace_log_sizes, interaction_log_sizes]
    }

    fn mix_into(self: @Claim, ref channel: Channel) {
        channel.mix_u64((*(self.log_size)).into());
        channel.mix_u64((*self.pedersen_builtin_segment_start).into());
    }
}

#[derive(Drop, Serde, Copy)]
pub struct InteractionClaim {
    pub claimed_sum: QM31,
}

#[generate_trait]
pub impl InteractionClaimImpl of InteractionClaimTrait {
    fn mix_into(self: @InteractionClaim, ref channel: Channel) {
        channel.mix_felts([*self.claimed_sum].span());
    }
}


#[derive(Drop)]
pub struct Component {
    pub claim: Claim,
    pub interaction_claim: InteractionClaim,
    pub range_check_5_4_lookup_elements: crate::RangeCheck_5_4Elements,
    pub memory_address_to_id_lookup_elements: crate::MemoryAddressToIdElements,
    pub memory_id_to_big_lookup_elements: crate::MemoryIdToBigElements,
    pub range_check_8_lookup_elements: crate::RangeCheck_8Elements,
    pub partial_ec_mul_lookup_elements: crate::PartialEcMulElements,
}

pub impl ComponentImpl of CairoComponent<Component> {
    fn mask_points(
        self: @Component,
        ref preprocessed_column_set: PreprocessedColumnSet,
        ref trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        let log_size = *(self.claim.log_size);
        let trace_gen = CanonicCosetImpl::new(log_size).coset.step;
        let point_offset_neg_1 = point.add_circle_point_m31(-trace_gen.mul(1).to_point());
        preprocessed_column_set.insert(PreprocessedColumn::Seq(*(self.claim.log_size)));
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
        interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
        interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
        interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
    }

    fn max_constraint_log_degree_bound(self: @Component) -> u32 {
        *(self.claim.log_size) + 1
    }

    fn evaluate_constraints_at_point(
        self: @Component,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
        point: CirclePoint<QM31>,
    ) {
        let log_size = *(self.claim.log_size);
        let trace_domain = CanonicCosetImpl::new(log_size);
        let domain_vanishing_eval_inv = trace_domain.eval_vanishing(point).inverse();
        let claimed_sum = *self.interaction_claim.claimed_sum;
        let column_size = m31(pow2(log_size));
        let pedersen_builtin_segment_start: QM31 = (TryInto::<
            u32, M31,
        >::try_into((*(self.claim.pedersen_builtin_segment_start)))
            .expect('pedersen_builtin.cairo:502'))
            .into();
        let mut range_check_5_4_sum_0: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_1: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_2: QM31 = Zero::zero();
        let mut range_check_5_4_sum_3: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_4: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_5: QM31 = Zero::zero();
        let mut range_check_8_sum_6: QM31 = Zero::zero();
        let mut range_check_8_sum_7: QM31 = Zero::zero();
        let mut range_check_8_sum_8: QM31 = Zero::zero();
        let mut range_check_8_sum_9: QM31 = Zero::zero();
        let mut partial_ec_mul_sum_10: QM31 = Zero::zero();
        let mut partial_ec_mul_sum_11: QM31 = Zero::zero();
        let mut partial_ec_mul_sum_12: QM31 = Zero::zero();
        let mut partial_ec_mul_sum_13: QM31 = Zero::zero();
        let mut partial_ec_mul_sum_14: QM31 = Zero::zero();
        let mut partial_ec_mul_sum_15: QM31 = Zero::zero();
        let mut partial_ec_mul_sum_16: QM31 = Zero::zero();
        let mut partial_ec_mul_sum_17: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_18: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_19: QM31 = Zero::zero();
        let seq = preprocessed_mask_values.get(PreprocessedColumn::Seq(*(self.claim.log_size)));

        let [
            value_limb_0_col0,
            value_limb_1_col1,
            value_limb_2_col2,
            value_limb_3_col3,
            value_limb_4_col4,
            value_limb_5_col5,
            value_limb_6_col6,
            value_limb_7_col7,
            value_limb_8_col8,
            value_limb_9_col9,
            value_limb_10_col10,
            value_limb_11_col11,
            value_limb_12_col12,
            value_limb_13_col13,
            value_limb_14_col14,
            value_limb_15_col15,
            value_limb_16_col16,
            value_limb_17_col17,
            value_limb_18_col18,
            value_limb_19_col19,
            value_limb_20_col20,
            value_limb_21_col21,
            value_limb_22_col22,
            value_limb_23_col23,
            value_limb_24_col24,
            value_limb_25_col25,
            value_limb_26_col26,
            ms_limb_low_col27,
            ms_limb_high_col28,
            pedersen_a_id_col29,
            value_limb_0_col30,
            value_limb_1_col31,
            value_limb_2_col32,
            value_limb_3_col33,
            value_limb_4_col34,
            value_limb_5_col35,
            value_limb_6_col36,
            value_limb_7_col37,
            value_limb_8_col38,
            value_limb_9_col39,
            value_limb_10_col40,
            value_limb_11_col41,
            value_limb_12_col42,
            value_limb_13_col43,
            value_limb_14_col44,
            value_limb_15_col45,
            value_limb_16_col46,
            value_limb_17_col47,
            value_limb_18_col48,
            value_limb_19_col49,
            value_limb_20_col50,
            value_limb_21_col51,
            value_limb_22_col52,
            value_limb_23_col53,
            value_limb_24_col54,
            value_limb_25_col55,
            value_limb_26_col56,
            ms_limb_low_col57,
            ms_limb_high_col58,
            pedersen_b_id_col59,
            ms_limb_is_max_col60,
            ms_and_mid_limbs_are_max_col61,
            rc_input_col62,
            ms_limb_is_max_col63,
            ms_and_mid_limbs_are_max_col64,
            rc_input_col65,
            partial_ec_mul_output_limb_0_col66,
            partial_ec_mul_output_limb_1_col67,
            partial_ec_mul_output_limb_2_col68,
            partial_ec_mul_output_limb_3_col69,
            partial_ec_mul_output_limb_4_col70,
            partial_ec_mul_output_limb_5_col71,
            partial_ec_mul_output_limb_6_col72,
            partial_ec_mul_output_limb_7_col73,
            partial_ec_mul_output_limb_8_col74,
            partial_ec_mul_output_limb_9_col75,
            partial_ec_mul_output_limb_10_col76,
            partial_ec_mul_output_limb_11_col77,
            partial_ec_mul_output_limb_12_col78,
            partial_ec_mul_output_limb_13_col79,
            partial_ec_mul_output_limb_14_col80,
            partial_ec_mul_output_limb_15_col81,
            partial_ec_mul_output_limb_16_col82,
            partial_ec_mul_output_limb_17_col83,
            partial_ec_mul_output_limb_18_col84,
            partial_ec_mul_output_limb_19_col85,
            partial_ec_mul_output_limb_20_col86,
            partial_ec_mul_output_limb_21_col87,
            partial_ec_mul_output_limb_22_col88,
            partial_ec_mul_output_limb_23_col89,
            partial_ec_mul_output_limb_24_col90,
            partial_ec_mul_output_limb_25_col91,
            partial_ec_mul_output_limb_26_col92,
            partial_ec_mul_output_limb_27_col93,
            partial_ec_mul_output_limb_28_col94,
            partial_ec_mul_output_limb_29_col95,
            partial_ec_mul_output_limb_30_col96,
            partial_ec_mul_output_limb_31_col97,
            partial_ec_mul_output_limb_32_col98,
            partial_ec_mul_output_limb_33_col99,
            partial_ec_mul_output_limb_34_col100,
            partial_ec_mul_output_limb_35_col101,
            partial_ec_mul_output_limb_36_col102,
            partial_ec_mul_output_limb_37_col103,
            partial_ec_mul_output_limb_38_col104,
            partial_ec_mul_output_limb_39_col105,
            partial_ec_mul_output_limb_40_col106,
            partial_ec_mul_output_limb_41_col107,
            partial_ec_mul_output_limb_42_col108,
            partial_ec_mul_output_limb_43_col109,
            partial_ec_mul_output_limb_44_col110,
            partial_ec_mul_output_limb_45_col111,
            partial_ec_mul_output_limb_46_col112,
            partial_ec_mul_output_limb_47_col113,
            partial_ec_mul_output_limb_48_col114,
            partial_ec_mul_output_limb_49_col115,
            partial_ec_mul_output_limb_50_col116,
            partial_ec_mul_output_limb_51_col117,
            partial_ec_mul_output_limb_52_col118,
            partial_ec_mul_output_limb_53_col119,
            partial_ec_mul_output_limb_54_col120,
            partial_ec_mul_output_limb_55_col121,
            partial_ec_mul_output_limb_56_col122,
            partial_ec_mul_output_limb_57_col123,
            partial_ec_mul_output_limb_58_col124,
            partial_ec_mul_output_limb_59_col125,
            partial_ec_mul_output_limb_60_col126,
            partial_ec_mul_output_limb_61_col127,
            partial_ec_mul_output_limb_62_col128,
            partial_ec_mul_output_limb_63_col129,
            partial_ec_mul_output_limb_64_col130,
            partial_ec_mul_output_limb_65_col131,
            partial_ec_mul_output_limb_66_col132,
            partial_ec_mul_output_limb_67_col133,
            partial_ec_mul_output_limb_68_col134,
            partial_ec_mul_output_limb_69_col135,
            partial_ec_mul_output_limb_70_col136,
            partial_ec_mul_output_limb_0_col137,
            partial_ec_mul_output_limb_1_col138,
            partial_ec_mul_output_limb_2_col139,
            partial_ec_mul_output_limb_3_col140,
            partial_ec_mul_output_limb_4_col141,
            partial_ec_mul_output_limb_5_col142,
            partial_ec_mul_output_limb_6_col143,
            partial_ec_mul_output_limb_7_col144,
            partial_ec_mul_output_limb_8_col145,
            partial_ec_mul_output_limb_9_col146,
            partial_ec_mul_output_limb_10_col147,
            partial_ec_mul_output_limb_11_col148,
            partial_ec_mul_output_limb_12_col149,
            partial_ec_mul_output_limb_13_col150,
            partial_ec_mul_output_limb_14_col151,
            partial_ec_mul_output_limb_15_col152,
            partial_ec_mul_output_limb_16_col153,
            partial_ec_mul_output_limb_17_col154,
            partial_ec_mul_output_limb_18_col155,
            partial_ec_mul_output_limb_19_col156,
            partial_ec_mul_output_limb_20_col157,
            partial_ec_mul_output_limb_21_col158,
            partial_ec_mul_output_limb_22_col159,
            partial_ec_mul_output_limb_23_col160,
            partial_ec_mul_output_limb_24_col161,
            partial_ec_mul_output_limb_25_col162,
            partial_ec_mul_output_limb_26_col163,
            partial_ec_mul_output_limb_27_col164,
            partial_ec_mul_output_limb_28_col165,
            partial_ec_mul_output_limb_29_col166,
            partial_ec_mul_output_limb_30_col167,
            partial_ec_mul_output_limb_31_col168,
            partial_ec_mul_output_limb_32_col169,
            partial_ec_mul_output_limb_33_col170,
            partial_ec_mul_output_limb_34_col171,
            partial_ec_mul_output_limb_35_col172,
            partial_ec_mul_output_limb_36_col173,
            partial_ec_mul_output_limb_37_col174,
            partial_ec_mul_output_limb_38_col175,
            partial_ec_mul_output_limb_39_col176,
            partial_ec_mul_output_limb_40_col177,
            partial_ec_mul_output_limb_41_col178,
            partial_ec_mul_output_limb_42_col179,
            partial_ec_mul_output_limb_43_col180,
            partial_ec_mul_output_limb_44_col181,
            partial_ec_mul_output_limb_45_col182,
            partial_ec_mul_output_limb_46_col183,
            partial_ec_mul_output_limb_47_col184,
            partial_ec_mul_output_limb_48_col185,
            partial_ec_mul_output_limb_49_col186,
            partial_ec_mul_output_limb_50_col187,
            partial_ec_mul_output_limb_51_col188,
            partial_ec_mul_output_limb_52_col189,
            partial_ec_mul_output_limb_53_col190,
            partial_ec_mul_output_limb_54_col191,
            partial_ec_mul_output_limb_55_col192,
            partial_ec_mul_output_limb_56_col193,
            partial_ec_mul_output_limb_57_col194,
            partial_ec_mul_output_limb_58_col195,
            partial_ec_mul_output_limb_59_col196,
            partial_ec_mul_output_limb_60_col197,
            partial_ec_mul_output_limb_61_col198,
            partial_ec_mul_output_limb_62_col199,
            partial_ec_mul_output_limb_63_col200,
            partial_ec_mul_output_limb_64_col201,
            partial_ec_mul_output_limb_65_col202,
            partial_ec_mul_output_limb_66_col203,
            partial_ec_mul_output_limb_67_col204,
            partial_ec_mul_output_limb_68_col205,
            partial_ec_mul_output_limb_69_col206,
            partial_ec_mul_output_limb_70_col207,
            partial_ec_mul_output_limb_0_col208,
            partial_ec_mul_output_limb_1_col209,
            partial_ec_mul_output_limb_2_col210,
            partial_ec_mul_output_limb_3_col211,
            partial_ec_mul_output_limb_4_col212,
            partial_ec_mul_output_limb_5_col213,
            partial_ec_mul_output_limb_6_col214,
            partial_ec_mul_output_limb_7_col215,
            partial_ec_mul_output_limb_8_col216,
            partial_ec_mul_output_limb_9_col217,
            partial_ec_mul_output_limb_10_col218,
            partial_ec_mul_output_limb_11_col219,
            partial_ec_mul_output_limb_12_col220,
            partial_ec_mul_output_limb_13_col221,
            partial_ec_mul_output_limb_14_col222,
            partial_ec_mul_output_limb_15_col223,
            partial_ec_mul_output_limb_16_col224,
            partial_ec_mul_output_limb_17_col225,
            partial_ec_mul_output_limb_18_col226,
            partial_ec_mul_output_limb_19_col227,
            partial_ec_mul_output_limb_20_col228,
            partial_ec_mul_output_limb_21_col229,
            partial_ec_mul_output_limb_22_col230,
            partial_ec_mul_output_limb_23_col231,
            partial_ec_mul_output_limb_24_col232,
            partial_ec_mul_output_limb_25_col233,
            partial_ec_mul_output_limb_26_col234,
            partial_ec_mul_output_limb_27_col235,
            partial_ec_mul_output_limb_28_col236,
            partial_ec_mul_output_limb_29_col237,
            partial_ec_mul_output_limb_30_col238,
            partial_ec_mul_output_limb_31_col239,
            partial_ec_mul_output_limb_32_col240,
            partial_ec_mul_output_limb_33_col241,
            partial_ec_mul_output_limb_34_col242,
            partial_ec_mul_output_limb_35_col243,
            partial_ec_mul_output_limb_36_col244,
            partial_ec_mul_output_limb_37_col245,
            partial_ec_mul_output_limb_38_col246,
            partial_ec_mul_output_limb_39_col247,
            partial_ec_mul_output_limb_40_col248,
            partial_ec_mul_output_limb_41_col249,
            partial_ec_mul_output_limb_42_col250,
            partial_ec_mul_output_limb_43_col251,
            partial_ec_mul_output_limb_44_col252,
            partial_ec_mul_output_limb_45_col253,
            partial_ec_mul_output_limb_46_col254,
            partial_ec_mul_output_limb_47_col255,
            partial_ec_mul_output_limb_48_col256,
            partial_ec_mul_output_limb_49_col257,
            partial_ec_mul_output_limb_50_col258,
            partial_ec_mul_output_limb_51_col259,
            partial_ec_mul_output_limb_52_col260,
            partial_ec_mul_output_limb_53_col261,
            partial_ec_mul_output_limb_54_col262,
            partial_ec_mul_output_limb_55_col263,
            partial_ec_mul_output_limb_56_col264,
            partial_ec_mul_output_limb_57_col265,
            partial_ec_mul_output_limb_58_col266,
            partial_ec_mul_output_limb_59_col267,
            partial_ec_mul_output_limb_60_col268,
            partial_ec_mul_output_limb_61_col269,
            partial_ec_mul_output_limb_62_col270,
            partial_ec_mul_output_limb_63_col271,
            partial_ec_mul_output_limb_64_col272,
            partial_ec_mul_output_limb_65_col273,
            partial_ec_mul_output_limb_66_col274,
            partial_ec_mul_output_limb_67_col275,
            partial_ec_mul_output_limb_68_col276,
            partial_ec_mul_output_limb_69_col277,
            partial_ec_mul_output_limb_70_col278,
            partial_ec_mul_output_limb_0_col279,
            partial_ec_mul_output_limb_1_col280,
            partial_ec_mul_output_limb_2_col281,
            partial_ec_mul_output_limb_3_col282,
            partial_ec_mul_output_limb_4_col283,
            partial_ec_mul_output_limb_5_col284,
            partial_ec_mul_output_limb_6_col285,
            partial_ec_mul_output_limb_7_col286,
            partial_ec_mul_output_limb_8_col287,
            partial_ec_mul_output_limb_9_col288,
            partial_ec_mul_output_limb_10_col289,
            partial_ec_mul_output_limb_11_col290,
            partial_ec_mul_output_limb_12_col291,
            partial_ec_mul_output_limb_13_col292,
            partial_ec_mul_output_limb_14_col293,
            partial_ec_mul_output_limb_15_col294,
            partial_ec_mul_output_limb_16_col295,
            partial_ec_mul_output_limb_17_col296,
            partial_ec_mul_output_limb_18_col297,
            partial_ec_mul_output_limb_19_col298,
            partial_ec_mul_output_limb_20_col299,
            partial_ec_mul_output_limb_21_col300,
            partial_ec_mul_output_limb_22_col301,
            partial_ec_mul_output_limb_23_col302,
            partial_ec_mul_output_limb_24_col303,
            partial_ec_mul_output_limb_25_col304,
            partial_ec_mul_output_limb_26_col305,
            partial_ec_mul_output_limb_27_col306,
            partial_ec_mul_output_limb_28_col307,
            partial_ec_mul_output_limb_29_col308,
            partial_ec_mul_output_limb_30_col309,
            partial_ec_mul_output_limb_31_col310,
            partial_ec_mul_output_limb_32_col311,
            partial_ec_mul_output_limb_33_col312,
            partial_ec_mul_output_limb_34_col313,
            partial_ec_mul_output_limb_35_col314,
            partial_ec_mul_output_limb_36_col315,
            partial_ec_mul_output_limb_37_col316,
            partial_ec_mul_output_limb_38_col317,
            partial_ec_mul_output_limb_39_col318,
            partial_ec_mul_output_limb_40_col319,
            partial_ec_mul_output_limb_41_col320,
            partial_ec_mul_output_limb_42_col321,
            partial_ec_mul_output_limb_43_col322,
            partial_ec_mul_output_limb_44_col323,
            partial_ec_mul_output_limb_45_col324,
            partial_ec_mul_output_limb_46_col325,
            partial_ec_mul_output_limb_47_col326,
            partial_ec_mul_output_limb_48_col327,
            partial_ec_mul_output_limb_49_col328,
            partial_ec_mul_output_limb_50_col329,
            partial_ec_mul_output_limb_51_col330,
            partial_ec_mul_output_limb_52_col331,
            partial_ec_mul_output_limb_53_col332,
            partial_ec_mul_output_limb_54_col333,
            partial_ec_mul_output_limb_55_col334,
            partial_ec_mul_output_limb_56_col335,
            partial_ec_mul_output_limb_57_col336,
            partial_ec_mul_output_limb_58_col337,
            partial_ec_mul_output_limb_59_col338,
            partial_ec_mul_output_limb_60_col339,
            partial_ec_mul_output_limb_61_col340,
            partial_ec_mul_output_limb_62_col341,
            partial_ec_mul_output_limb_63_col342,
            partial_ec_mul_output_limb_64_col343,
            partial_ec_mul_output_limb_65_col344,
            partial_ec_mul_output_limb_66_col345,
            partial_ec_mul_output_limb_67_col346,
            partial_ec_mul_output_limb_68_col347,
            partial_ec_mul_output_limb_69_col348,
            partial_ec_mul_output_limb_70_col349,
            pedersen_result_id_col350,
        ]: [Span<QM31>; 351] =
            (*trace_mask_values
            .multi_pop_front()
            .unwrap())
            .unbox();
        let [value_limb_0_col0]: [QM31; 1] = (*value_limb_0_col0.try_into().unwrap()).unbox();
        let [value_limb_1_col1]: [QM31; 1] = (*value_limb_1_col1.try_into().unwrap()).unbox();
        let [value_limb_2_col2]: [QM31; 1] = (*value_limb_2_col2.try_into().unwrap()).unbox();
        let [value_limb_3_col3]: [QM31; 1] = (*value_limb_3_col3.try_into().unwrap()).unbox();
        let [value_limb_4_col4]: [QM31; 1] = (*value_limb_4_col4.try_into().unwrap()).unbox();
        let [value_limb_5_col5]: [QM31; 1] = (*value_limb_5_col5.try_into().unwrap()).unbox();
        let [value_limb_6_col6]: [QM31; 1] = (*value_limb_6_col6.try_into().unwrap()).unbox();
        let [value_limb_7_col7]: [QM31; 1] = (*value_limb_7_col7.try_into().unwrap()).unbox();
        let [value_limb_8_col8]: [QM31; 1] = (*value_limb_8_col8.try_into().unwrap()).unbox();
        let [value_limb_9_col9]: [QM31; 1] = (*value_limb_9_col9.try_into().unwrap()).unbox();
        let [value_limb_10_col10]: [QM31; 1] = (*value_limb_10_col10.try_into().unwrap()).unbox();
        let [value_limb_11_col11]: [QM31; 1] = (*value_limb_11_col11.try_into().unwrap()).unbox();
        let [value_limb_12_col12]: [QM31; 1] = (*value_limb_12_col12.try_into().unwrap()).unbox();
        let [value_limb_13_col13]: [QM31; 1] = (*value_limb_13_col13.try_into().unwrap()).unbox();
        let [value_limb_14_col14]: [QM31; 1] = (*value_limb_14_col14.try_into().unwrap()).unbox();
        let [value_limb_15_col15]: [QM31; 1] = (*value_limb_15_col15.try_into().unwrap()).unbox();
        let [value_limb_16_col16]: [QM31; 1] = (*value_limb_16_col16.try_into().unwrap()).unbox();
        let [value_limb_17_col17]: [QM31; 1] = (*value_limb_17_col17.try_into().unwrap()).unbox();
        let [value_limb_18_col18]: [QM31; 1] = (*value_limb_18_col18.try_into().unwrap()).unbox();
        let [value_limb_19_col19]: [QM31; 1] = (*value_limb_19_col19.try_into().unwrap()).unbox();
        let [value_limb_20_col20]: [QM31; 1] = (*value_limb_20_col20.try_into().unwrap()).unbox();
        let [value_limb_21_col21]: [QM31; 1] = (*value_limb_21_col21.try_into().unwrap()).unbox();
        let [value_limb_22_col22]: [QM31; 1] = (*value_limb_22_col22.try_into().unwrap()).unbox();
        let [value_limb_23_col23]: [QM31; 1] = (*value_limb_23_col23.try_into().unwrap()).unbox();
        let [value_limb_24_col24]: [QM31; 1] = (*value_limb_24_col24.try_into().unwrap()).unbox();
        let [value_limb_25_col25]: [QM31; 1] = (*value_limb_25_col25.try_into().unwrap()).unbox();
        let [value_limb_26_col26]: [QM31; 1] = (*value_limb_26_col26.try_into().unwrap()).unbox();
        let [ms_limb_low_col27]: [QM31; 1] = (*ms_limb_low_col27.try_into().unwrap()).unbox();
        let [ms_limb_high_col28]: [QM31; 1] = (*ms_limb_high_col28.try_into().unwrap()).unbox();
        let [pedersen_a_id_col29]: [QM31; 1] = (*pedersen_a_id_col29.try_into().unwrap()).unbox();
        let [value_limb_0_col30]: [QM31; 1] = (*value_limb_0_col30.try_into().unwrap()).unbox();
        let [value_limb_1_col31]: [QM31; 1] = (*value_limb_1_col31.try_into().unwrap()).unbox();
        let [value_limb_2_col32]: [QM31; 1] = (*value_limb_2_col32.try_into().unwrap()).unbox();
        let [value_limb_3_col33]: [QM31; 1] = (*value_limb_3_col33.try_into().unwrap()).unbox();
        let [value_limb_4_col34]: [QM31; 1] = (*value_limb_4_col34.try_into().unwrap()).unbox();
        let [value_limb_5_col35]: [QM31; 1] = (*value_limb_5_col35.try_into().unwrap()).unbox();
        let [value_limb_6_col36]: [QM31; 1] = (*value_limb_6_col36.try_into().unwrap()).unbox();
        let [value_limb_7_col37]: [QM31; 1] = (*value_limb_7_col37.try_into().unwrap()).unbox();
        let [value_limb_8_col38]: [QM31; 1] = (*value_limb_8_col38.try_into().unwrap()).unbox();
        let [value_limb_9_col39]: [QM31; 1] = (*value_limb_9_col39.try_into().unwrap()).unbox();
        let [value_limb_10_col40]: [QM31; 1] = (*value_limb_10_col40.try_into().unwrap()).unbox();
        let [value_limb_11_col41]: [QM31; 1] = (*value_limb_11_col41.try_into().unwrap()).unbox();
        let [value_limb_12_col42]: [QM31; 1] = (*value_limb_12_col42.try_into().unwrap()).unbox();
        let [value_limb_13_col43]: [QM31; 1] = (*value_limb_13_col43.try_into().unwrap()).unbox();
        let [value_limb_14_col44]: [QM31; 1] = (*value_limb_14_col44.try_into().unwrap()).unbox();
        let [value_limb_15_col45]: [QM31; 1] = (*value_limb_15_col45.try_into().unwrap()).unbox();
        let [value_limb_16_col46]: [QM31; 1] = (*value_limb_16_col46.try_into().unwrap()).unbox();
        let [value_limb_17_col47]: [QM31; 1] = (*value_limb_17_col47.try_into().unwrap()).unbox();
        let [value_limb_18_col48]: [QM31; 1] = (*value_limb_18_col48.try_into().unwrap()).unbox();
        let [value_limb_19_col49]: [QM31; 1] = (*value_limb_19_col49.try_into().unwrap()).unbox();
        let [value_limb_20_col50]: [QM31; 1] = (*value_limb_20_col50.try_into().unwrap()).unbox();
        let [value_limb_21_col51]: [QM31; 1] = (*value_limb_21_col51.try_into().unwrap()).unbox();
        let [value_limb_22_col52]: [QM31; 1] = (*value_limb_22_col52.try_into().unwrap()).unbox();
        let [value_limb_23_col53]: [QM31; 1] = (*value_limb_23_col53.try_into().unwrap()).unbox();
        let [value_limb_24_col54]: [QM31; 1] = (*value_limb_24_col54.try_into().unwrap()).unbox();
        let [value_limb_25_col55]: [QM31; 1] = (*value_limb_25_col55.try_into().unwrap()).unbox();
        let [value_limb_26_col56]: [QM31; 1] = (*value_limb_26_col56.try_into().unwrap()).unbox();
        let [ms_limb_low_col57]: [QM31; 1] = (*ms_limb_low_col57.try_into().unwrap()).unbox();
        let [ms_limb_high_col58]: [QM31; 1] = (*ms_limb_high_col58.try_into().unwrap()).unbox();
        let [pedersen_b_id_col59]: [QM31; 1] = (*pedersen_b_id_col59.try_into().unwrap()).unbox();
        let [ms_limb_is_max_col60]: [QM31; 1] = (*ms_limb_is_max_col60.try_into().unwrap()).unbox();
        let [ms_and_mid_limbs_are_max_col61]: [QM31; 1] = (*ms_and_mid_limbs_are_max_col61
            .try_into()
            .unwrap())
            .unbox();
        let [rc_input_col62]: [QM31; 1] = (*rc_input_col62.try_into().unwrap()).unbox();
        let [ms_limb_is_max_col63]: [QM31; 1] = (*ms_limb_is_max_col63.try_into().unwrap()).unbox();
        let [ms_and_mid_limbs_are_max_col64]: [QM31; 1] = (*ms_and_mid_limbs_are_max_col64
            .try_into()
            .unwrap())
            .unbox();
        let [rc_input_col65]: [QM31; 1] = (*rc_input_col65.try_into().unwrap()).unbox();
        let [partial_ec_mul_output_limb_0_col66]: [QM31; 1] = (*partial_ec_mul_output_limb_0_col66
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_1_col67]: [QM31; 1] = (*partial_ec_mul_output_limb_1_col67
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_2_col68]: [QM31; 1] = (*partial_ec_mul_output_limb_2_col68
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_3_col69]: [QM31; 1] = (*partial_ec_mul_output_limb_3_col69
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_4_col70]: [QM31; 1] = (*partial_ec_mul_output_limb_4_col70
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_5_col71]: [QM31; 1] = (*partial_ec_mul_output_limb_5_col71
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_6_col72]: [QM31; 1] = (*partial_ec_mul_output_limb_6_col72
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_7_col73]: [QM31; 1] = (*partial_ec_mul_output_limb_7_col73
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_8_col74]: [QM31; 1] = (*partial_ec_mul_output_limb_8_col74
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_9_col75]: [QM31; 1] = (*partial_ec_mul_output_limb_9_col75
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_10_col76]: [QM31; 1] = (*partial_ec_mul_output_limb_10_col76
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_11_col77]: [QM31; 1] = (*partial_ec_mul_output_limb_11_col77
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_12_col78]: [QM31; 1] = (*partial_ec_mul_output_limb_12_col78
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_13_col79]: [QM31; 1] = (*partial_ec_mul_output_limb_13_col79
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_14_col80]: [QM31; 1] = (*partial_ec_mul_output_limb_14_col80
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_15_col81]: [QM31; 1] = (*partial_ec_mul_output_limb_15_col81
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_16_col82]: [QM31; 1] = (*partial_ec_mul_output_limb_16_col82
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_17_col83]: [QM31; 1] = (*partial_ec_mul_output_limb_17_col83
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_18_col84]: [QM31; 1] = (*partial_ec_mul_output_limb_18_col84
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_19_col85]: [QM31; 1] = (*partial_ec_mul_output_limb_19_col85
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_20_col86]: [QM31; 1] = (*partial_ec_mul_output_limb_20_col86
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_21_col87]: [QM31; 1] = (*partial_ec_mul_output_limb_21_col87
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_22_col88]: [QM31; 1] = (*partial_ec_mul_output_limb_22_col88
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_23_col89]: [QM31; 1] = (*partial_ec_mul_output_limb_23_col89
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_24_col90]: [QM31; 1] = (*partial_ec_mul_output_limb_24_col90
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_25_col91]: [QM31; 1] = (*partial_ec_mul_output_limb_25_col91
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_26_col92]: [QM31; 1] = (*partial_ec_mul_output_limb_26_col92
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_27_col93]: [QM31; 1] = (*partial_ec_mul_output_limb_27_col93
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_28_col94]: [QM31; 1] = (*partial_ec_mul_output_limb_28_col94
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_29_col95]: [QM31; 1] = (*partial_ec_mul_output_limb_29_col95
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_30_col96]: [QM31; 1] = (*partial_ec_mul_output_limb_30_col96
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_31_col97]: [QM31; 1] = (*partial_ec_mul_output_limb_31_col97
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_32_col98]: [QM31; 1] = (*partial_ec_mul_output_limb_32_col98
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_33_col99]: [QM31; 1] = (*partial_ec_mul_output_limb_33_col99
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_34_col100]: [QM31; 1] =
            (*partial_ec_mul_output_limb_34_col100
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_35_col101]: [QM31; 1] =
            (*partial_ec_mul_output_limb_35_col101
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_36_col102]: [QM31; 1] =
            (*partial_ec_mul_output_limb_36_col102
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_37_col103]: [QM31; 1] =
            (*partial_ec_mul_output_limb_37_col103
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_38_col104]: [QM31; 1] =
            (*partial_ec_mul_output_limb_38_col104
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_39_col105]: [QM31; 1] =
            (*partial_ec_mul_output_limb_39_col105
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_40_col106]: [QM31; 1] =
            (*partial_ec_mul_output_limb_40_col106
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_41_col107]: [QM31; 1] =
            (*partial_ec_mul_output_limb_41_col107
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_42_col108]: [QM31; 1] =
            (*partial_ec_mul_output_limb_42_col108
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_43_col109]: [QM31; 1] =
            (*partial_ec_mul_output_limb_43_col109
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_44_col110]: [QM31; 1] =
            (*partial_ec_mul_output_limb_44_col110
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_45_col111]: [QM31; 1] =
            (*partial_ec_mul_output_limb_45_col111
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_46_col112]: [QM31; 1] =
            (*partial_ec_mul_output_limb_46_col112
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_47_col113]: [QM31; 1] =
            (*partial_ec_mul_output_limb_47_col113
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_48_col114]: [QM31; 1] =
            (*partial_ec_mul_output_limb_48_col114
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_49_col115]: [QM31; 1] =
            (*partial_ec_mul_output_limb_49_col115
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_50_col116]: [QM31; 1] =
            (*partial_ec_mul_output_limb_50_col116
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_51_col117]: [QM31; 1] =
            (*partial_ec_mul_output_limb_51_col117
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_52_col118]: [QM31; 1] =
            (*partial_ec_mul_output_limb_52_col118
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_53_col119]: [QM31; 1] =
            (*partial_ec_mul_output_limb_53_col119
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_54_col120]: [QM31; 1] =
            (*partial_ec_mul_output_limb_54_col120
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_55_col121]: [QM31; 1] =
            (*partial_ec_mul_output_limb_55_col121
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_56_col122]: [QM31; 1] =
            (*partial_ec_mul_output_limb_56_col122
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_57_col123]: [QM31; 1] =
            (*partial_ec_mul_output_limb_57_col123
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_58_col124]: [QM31; 1] =
            (*partial_ec_mul_output_limb_58_col124
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_59_col125]: [QM31; 1] =
            (*partial_ec_mul_output_limb_59_col125
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_60_col126]: [QM31; 1] =
            (*partial_ec_mul_output_limb_60_col126
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_61_col127]: [QM31; 1] =
            (*partial_ec_mul_output_limb_61_col127
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_62_col128]: [QM31; 1] =
            (*partial_ec_mul_output_limb_62_col128
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_63_col129]: [QM31; 1] =
            (*partial_ec_mul_output_limb_63_col129
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_64_col130]: [QM31; 1] =
            (*partial_ec_mul_output_limb_64_col130
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_65_col131]: [QM31; 1] =
            (*partial_ec_mul_output_limb_65_col131
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_66_col132]: [QM31; 1] =
            (*partial_ec_mul_output_limb_66_col132
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_67_col133]: [QM31; 1] =
            (*partial_ec_mul_output_limb_67_col133
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_68_col134]: [QM31; 1] =
            (*partial_ec_mul_output_limb_68_col134
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_69_col135]: [QM31; 1] =
            (*partial_ec_mul_output_limb_69_col135
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_70_col136]: [QM31; 1] =
            (*partial_ec_mul_output_limb_70_col136
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_0_col137]: [QM31; 1] = (*partial_ec_mul_output_limb_0_col137
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_1_col138]: [QM31; 1] = (*partial_ec_mul_output_limb_1_col138
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_2_col139]: [QM31; 1] = (*partial_ec_mul_output_limb_2_col139
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_3_col140]: [QM31; 1] = (*partial_ec_mul_output_limb_3_col140
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_4_col141]: [QM31; 1] = (*partial_ec_mul_output_limb_4_col141
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_5_col142]: [QM31; 1] = (*partial_ec_mul_output_limb_5_col142
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_6_col143]: [QM31; 1] = (*partial_ec_mul_output_limb_6_col143
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_7_col144]: [QM31; 1] = (*partial_ec_mul_output_limb_7_col144
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_8_col145]: [QM31; 1] = (*partial_ec_mul_output_limb_8_col145
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_9_col146]: [QM31; 1] = (*partial_ec_mul_output_limb_9_col146
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_10_col147]: [QM31; 1] =
            (*partial_ec_mul_output_limb_10_col147
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_11_col148]: [QM31; 1] =
            (*partial_ec_mul_output_limb_11_col148
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_12_col149]: [QM31; 1] =
            (*partial_ec_mul_output_limb_12_col149
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_13_col150]: [QM31; 1] =
            (*partial_ec_mul_output_limb_13_col150
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_14_col151]: [QM31; 1] =
            (*partial_ec_mul_output_limb_14_col151
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_15_col152]: [QM31; 1] =
            (*partial_ec_mul_output_limb_15_col152
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_16_col153]: [QM31; 1] =
            (*partial_ec_mul_output_limb_16_col153
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_17_col154]: [QM31; 1] =
            (*partial_ec_mul_output_limb_17_col154
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_18_col155]: [QM31; 1] =
            (*partial_ec_mul_output_limb_18_col155
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_19_col156]: [QM31; 1] =
            (*partial_ec_mul_output_limb_19_col156
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_20_col157]: [QM31; 1] =
            (*partial_ec_mul_output_limb_20_col157
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_21_col158]: [QM31; 1] =
            (*partial_ec_mul_output_limb_21_col158
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_22_col159]: [QM31; 1] =
            (*partial_ec_mul_output_limb_22_col159
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_23_col160]: [QM31; 1] =
            (*partial_ec_mul_output_limb_23_col160
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_24_col161]: [QM31; 1] =
            (*partial_ec_mul_output_limb_24_col161
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_25_col162]: [QM31; 1] =
            (*partial_ec_mul_output_limb_25_col162
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_26_col163]: [QM31; 1] =
            (*partial_ec_mul_output_limb_26_col163
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_27_col164]: [QM31; 1] =
            (*partial_ec_mul_output_limb_27_col164
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_28_col165]: [QM31; 1] =
            (*partial_ec_mul_output_limb_28_col165
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_29_col166]: [QM31; 1] =
            (*partial_ec_mul_output_limb_29_col166
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_30_col167]: [QM31; 1] =
            (*partial_ec_mul_output_limb_30_col167
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_31_col168]: [QM31; 1] =
            (*partial_ec_mul_output_limb_31_col168
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_32_col169]: [QM31; 1] =
            (*partial_ec_mul_output_limb_32_col169
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_33_col170]: [QM31; 1] =
            (*partial_ec_mul_output_limb_33_col170
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_34_col171]: [QM31; 1] =
            (*partial_ec_mul_output_limb_34_col171
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_35_col172]: [QM31; 1] =
            (*partial_ec_mul_output_limb_35_col172
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_36_col173]: [QM31; 1] =
            (*partial_ec_mul_output_limb_36_col173
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_37_col174]: [QM31; 1] =
            (*partial_ec_mul_output_limb_37_col174
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_38_col175]: [QM31; 1] =
            (*partial_ec_mul_output_limb_38_col175
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_39_col176]: [QM31; 1] =
            (*partial_ec_mul_output_limb_39_col176
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_40_col177]: [QM31; 1] =
            (*partial_ec_mul_output_limb_40_col177
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_41_col178]: [QM31; 1] =
            (*partial_ec_mul_output_limb_41_col178
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_42_col179]: [QM31; 1] =
            (*partial_ec_mul_output_limb_42_col179
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_43_col180]: [QM31; 1] =
            (*partial_ec_mul_output_limb_43_col180
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_44_col181]: [QM31; 1] =
            (*partial_ec_mul_output_limb_44_col181
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_45_col182]: [QM31; 1] =
            (*partial_ec_mul_output_limb_45_col182
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_46_col183]: [QM31; 1] =
            (*partial_ec_mul_output_limb_46_col183
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_47_col184]: [QM31; 1] =
            (*partial_ec_mul_output_limb_47_col184
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_48_col185]: [QM31; 1] =
            (*partial_ec_mul_output_limb_48_col185
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_49_col186]: [QM31; 1] =
            (*partial_ec_mul_output_limb_49_col186
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_50_col187]: [QM31; 1] =
            (*partial_ec_mul_output_limb_50_col187
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_51_col188]: [QM31; 1] =
            (*partial_ec_mul_output_limb_51_col188
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_52_col189]: [QM31; 1] =
            (*partial_ec_mul_output_limb_52_col189
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_53_col190]: [QM31; 1] =
            (*partial_ec_mul_output_limb_53_col190
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_54_col191]: [QM31; 1] =
            (*partial_ec_mul_output_limb_54_col191
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_55_col192]: [QM31; 1] =
            (*partial_ec_mul_output_limb_55_col192
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_56_col193]: [QM31; 1] =
            (*partial_ec_mul_output_limb_56_col193
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_57_col194]: [QM31; 1] =
            (*partial_ec_mul_output_limb_57_col194
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_58_col195]: [QM31; 1] =
            (*partial_ec_mul_output_limb_58_col195
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_59_col196]: [QM31; 1] =
            (*partial_ec_mul_output_limb_59_col196
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_60_col197]: [QM31; 1] =
            (*partial_ec_mul_output_limb_60_col197
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_61_col198]: [QM31; 1] =
            (*partial_ec_mul_output_limb_61_col198
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_62_col199]: [QM31; 1] =
            (*partial_ec_mul_output_limb_62_col199
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_63_col200]: [QM31; 1] =
            (*partial_ec_mul_output_limb_63_col200
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_64_col201]: [QM31; 1] =
            (*partial_ec_mul_output_limb_64_col201
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_65_col202]: [QM31; 1] =
            (*partial_ec_mul_output_limb_65_col202
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_66_col203]: [QM31; 1] =
            (*partial_ec_mul_output_limb_66_col203
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_67_col204]: [QM31; 1] =
            (*partial_ec_mul_output_limb_67_col204
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_68_col205]: [QM31; 1] =
            (*partial_ec_mul_output_limb_68_col205
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_69_col206]: [QM31; 1] =
            (*partial_ec_mul_output_limb_69_col206
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_70_col207]: [QM31; 1] =
            (*partial_ec_mul_output_limb_70_col207
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_0_col208]: [QM31; 1] = (*partial_ec_mul_output_limb_0_col208
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_1_col209]: [QM31; 1] = (*partial_ec_mul_output_limb_1_col209
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_2_col210]: [QM31; 1] = (*partial_ec_mul_output_limb_2_col210
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_3_col211]: [QM31; 1] = (*partial_ec_mul_output_limb_3_col211
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_4_col212]: [QM31; 1] = (*partial_ec_mul_output_limb_4_col212
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_5_col213]: [QM31; 1] = (*partial_ec_mul_output_limb_5_col213
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_6_col214]: [QM31; 1] = (*partial_ec_mul_output_limb_6_col214
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_7_col215]: [QM31; 1] = (*partial_ec_mul_output_limb_7_col215
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_8_col216]: [QM31; 1] = (*partial_ec_mul_output_limb_8_col216
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_9_col217]: [QM31; 1] = (*partial_ec_mul_output_limb_9_col217
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_10_col218]: [QM31; 1] =
            (*partial_ec_mul_output_limb_10_col218
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_11_col219]: [QM31; 1] =
            (*partial_ec_mul_output_limb_11_col219
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_12_col220]: [QM31; 1] =
            (*partial_ec_mul_output_limb_12_col220
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_13_col221]: [QM31; 1] =
            (*partial_ec_mul_output_limb_13_col221
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_14_col222]: [QM31; 1] =
            (*partial_ec_mul_output_limb_14_col222
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_15_col223]: [QM31; 1] =
            (*partial_ec_mul_output_limb_15_col223
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_16_col224]: [QM31; 1] =
            (*partial_ec_mul_output_limb_16_col224
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_17_col225]: [QM31; 1] =
            (*partial_ec_mul_output_limb_17_col225
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_18_col226]: [QM31; 1] =
            (*partial_ec_mul_output_limb_18_col226
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_19_col227]: [QM31; 1] =
            (*partial_ec_mul_output_limb_19_col227
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_20_col228]: [QM31; 1] =
            (*partial_ec_mul_output_limb_20_col228
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_21_col229]: [QM31; 1] =
            (*partial_ec_mul_output_limb_21_col229
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_22_col230]: [QM31; 1] =
            (*partial_ec_mul_output_limb_22_col230
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_23_col231]: [QM31; 1] =
            (*partial_ec_mul_output_limb_23_col231
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_24_col232]: [QM31; 1] =
            (*partial_ec_mul_output_limb_24_col232
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_25_col233]: [QM31; 1] =
            (*partial_ec_mul_output_limb_25_col233
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_26_col234]: [QM31; 1] =
            (*partial_ec_mul_output_limb_26_col234
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_27_col235]: [QM31; 1] =
            (*partial_ec_mul_output_limb_27_col235
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_28_col236]: [QM31; 1] =
            (*partial_ec_mul_output_limb_28_col236
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_29_col237]: [QM31; 1] =
            (*partial_ec_mul_output_limb_29_col237
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_30_col238]: [QM31; 1] =
            (*partial_ec_mul_output_limb_30_col238
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_31_col239]: [QM31; 1] =
            (*partial_ec_mul_output_limb_31_col239
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_32_col240]: [QM31; 1] =
            (*partial_ec_mul_output_limb_32_col240
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_33_col241]: [QM31; 1] =
            (*partial_ec_mul_output_limb_33_col241
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_34_col242]: [QM31; 1] =
            (*partial_ec_mul_output_limb_34_col242
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_35_col243]: [QM31; 1] =
            (*partial_ec_mul_output_limb_35_col243
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_36_col244]: [QM31; 1] =
            (*partial_ec_mul_output_limb_36_col244
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_37_col245]: [QM31; 1] =
            (*partial_ec_mul_output_limb_37_col245
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_38_col246]: [QM31; 1] =
            (*partial_ec_mul_output_limb_38_col246
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_39_col247]: [QM31; 1] =
            (*partial_ec_mul_output_limb_39_col247
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_40_col248]: [QM31; 1] =
            (*partial_ec_mul_output_limb_40_col248
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_41_col249]: [QM31; 1] =
            (*partial_ec_mul_output_limb_41_col249
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_42_col250]: [QM31; 1] =
            (*partial_ec_mul_output_limb_42_col250
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_43_col251]: [QM31; 1] =
            (*partial_ec_mul_output_limb_43_col251
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_44_col252]: [QM31; 1] =
            (*partial_ec_mul_output_limb_44_col252
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_45_col253]: [QM31; 1] =
            (*partial_ec_mul_output_limb_45_col253
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_46_col254]: [QM31; 1] =
            (*partial_ec_mul_output_limb_46_col254
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_47_col255]: [QM31; 1] =
            (*partial_ec_mul_output_limb_47_col255
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_48_col256]: [QM31; 1] =
            (*partial_ec_mul_output_limb_48_col256
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_49_col257]: [QM31; 1] =
            (*partial_ec_mul_output_limb_49_col257
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_50_col258]: [QM31; 1] =
            (*partial_ec_mul_output_limb_50_col258
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_51_col259]: [QM31; 1] =
            (*partial_ec_mul_output_limb_51_col259
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_52_col260]: [QM31; 1] =
            (*partial_ec_mul_output_limb_52_col260
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_53_col261]: [QM31; 1] =
            (*partial_ec_mul_output_limb_53_col261
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_54_col262]: [QM31; 1] =
            (*partial_ec_mul_output_limb_54_col262
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_55_col263]: [QM31; 1] =
            (*partial_ec_mul_output_limb_55_col263
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_56_col264]: [QM31; 1] =
            (*partial_ec_mul_output_limb_56_col264
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_57_col265]: [QM31; 1] =
            (*partial_ec_mul_output_limb_57_col265
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_58_col266]: [QM31; 1] =
            (*partial_ec_mul_output_limb_58_col266
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_59_col267]: [QM31; 1] =
            (*partial_ec_mul_output_limb_59_col267
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_60_col268]: [QM31; 1] =
            (*partial_ec_mul_output_limb_60_col268
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_61_col269]: [QM31; 1] =
            (*partial_ec_mul_output_limb_61_col269
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_62_col270]: [QM31; 1] =
            (*partial_ec_mul_output_limb_62_col270
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_63_col271]: [QM31; 1] =
            (*partial_ec_mul_output_limb_63_col271
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_64_col272]: [QM31; 1] =
            (*partial_ec_mul_output_limb_64_col272
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_65_col273]: [QM31; 1] =
            (*partial_ec_mul_output_limb_65_col273
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_66_col274]: [QM31; 1] =
            (*partial_ec_mul_output_limb_66_col274
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_67_col275]: [QM31; 1] =
            (*partial_ec_mul_output_limb_67_col275
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_68_col276]: [QM31; 1] =
            (*partial_ec_mul_output_limb_68_col276
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_69_col277]: [QM31; 1] =
            (*partial_ec_mul_output_limb_69_col277
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_70_col278]: [QM31; 1] =
            (*partial_ec_mul_output_limb_70_col278
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_0_col279]: [QM31; 1] = (*partial_ec_mul_output_limb_0_col279
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_1_col280]: [QM31; 1] = (*partial_ec_mul_output_limb_1_col280
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_2_col281]: [QM31; 1] = (*partial_ec_mul_output_limb_2_col281
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_3_col282]: [QM31; 1] = (*partial_ec_mul_output_limb_3_col282
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_4_col283]: [QM31; 1] = (*partial_ec_mul_output_limb_4_col283
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_5_col284]: [QM31; 1] = (*partial_ec_mul_output_limb_5_col284
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_6_col285]: [QM31; 1] = (*partial_ec_mul_output_limb_6_col285
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_7_col286]: [QM31; 1] = (*partial_ec_mul_output_limb_7_col286
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_8_col287]: [QM31; 1] = (*partial_ec_mul_output_limb_8_col287
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_9_col288]: [QM31; 1] = (*partial_ec_mul_output_limb_9_col288
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_10_col289]: [QM31; 1] =
            (*partial_ec_mul_output_limb_10_col289
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_11_col290]: [QM31; 1] =
            (*partial_ec_mul_output_limb_11_col290
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_12_col291]: [QM31; 1] =
            (*partial_ec_mul_output_limb_12_col291
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_13_col292]: [QM31; 1] =
            (*partial_ec_mul_output_limb_13_col292
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_14_col293]: [QM31; 1] =
            (*partial_ec_mul_output_limb_14_col293
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_15_col294]: [QM31; 1] =
            (*partial_ec_mul_output_limb_15_col294
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_16_col295]: [QM31; 1] =
            (*partial_ec_mul_output_limb_16_col295
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_17_col296]: [QM31; 1] =
            (*partial_ec_mul_output_limb_17_col296
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_18_col297]: [QM31; 1] =
            (*partial_ec_mul_output_limb_18_col297
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_19_col298]: [QM31; 1] =
            (*partial_ec_mul_output_limb_19_col298
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_20_col299]: [QM31; 1] =
            (*partial_ec_mul_output_limb_20_col299
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_21_col300]: [QM31; 1] =
            (*partial_ec_mul_output_limb_21_col300
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_22_col301]: [QM31; 1] =
            (*partial_ec_mul_output_limb_22_col301
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_23_col302]: [QM31; 1] =
            (*partial_ec_mul_output_limb_23_col302
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_24_col303]: [QM31; 1] =
            (*partial_ec_mul_output_limb_24_col303
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_25_col304]: [QM31; 1] =
            (*partial_ec_mul_output_limb_25_col304
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_26_col305]: [QM31; 1] =
            (*partial_ec_mul_output_limb_26_col305
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_27_col306]: [QM31; 1] =
            (*partial_ec_mul_output_limb_27_col306
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_28_col307]: [QM31; 1] =
            (*partial_ec_mul_output_limb_28_col307
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_29_col308]: [QM31; 1] =
            (*partial_ec_mul_output_limb_29_col308
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_30_col309]: [QM31; 1] =
            (*partial_ec_mul_output_limb_30_col309
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_31_col310]: [QM31; 1] =
            (*partial_ec_mul_output_limb_31_col310
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_32_col311]: [QM31; 1] =
            (*partial_ec_mul_output_limb_32_col311
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_33_col312]: [QM31; 1] =
            (*partial_ec_mul_output_limb_33_col312
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_34_col313]: [QM31; 1] =
            (*partial_ec_mul_output_limb_34_col313
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_35_col314]: [QM31; 1] =
            (*partial_ec_mul_output_limb_35_col314
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_36_col315]: [QM31; 1] =
            (*partial_ec_mul_output_limb_36_col315
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_37_col316]: [QM31; 1] =
            (*partial_ec_mul_output_limb_37_col316
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_38_col317]: [QM31; 1] =
            (*partial_ec_mul_output_limb_38_col317
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_39_col318]: [QM31; 1] =
            (*partial_ec_mul_output_limb_39_col318
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_40_col319]: [QM31; 1] =
            (*partial_ec_mul_output_limb_40_col319
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_41_col320]: [QM31; 1] =
            (*partial_ec_mul_output_limb_41_col320
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_42_col321]: [QM31; 1] =
            (*partial_ec_mul_output_limb_42_col321
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_43_col322]: [QM31; 1] =
            (*partial_ec_mul_output_limb_43_col322
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_44_col323]: [QM31; 1] =
            (*partial_ec_mul_output_limb_44_col323
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_45_col324]: [QM31; 1] =
            (*partial_ec_mul_output_limb_45_col324
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_46_col325]: [QM31; 1] =
            (*partial_ec_mul_output_limb_46_col325
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_47_col326]: [QM31; 1] =
            (*partial_ec_mul_output_limb_47_col326
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_48_col327]: [QM31; 1] =
            (*partial_ec_mul_output_limb_48_col327
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_49_col328]: [QM31; 1] =
            (*partial_ec_mul_output_limb_49_col328
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_50_col329]: [QM31; 1] =
            (*partial_ec_mul_output_limb_50_col329
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_51_col330]: [QM31; 1] =
            (*partial_ec_mul_output_limb_51_col330
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_52_col331]: [QM31; 1] =
            (*partial_ec_mul_output_limb_52_col331
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_53_col332]: [QM31; 1] =
            (*partial_ec_mul_output_limb_53_col332
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_54_col333]: [QM31; 1] =
            (*partial_ec_mul_output_limb_54_col333
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_55_col334]: [QM31; 1] =
            (*partial_ec_mul_output_limb_55_col334
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_56_col335]: [QM31; 1] =
            (*partial_ec_mul_output_limb_56_col335
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_57_col336]: [QM31; 1] =
            (*partial_ec_mul_output_limb_57_col336
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_58_col337]: [QM31; 1] =
            (*partial_ec_mul_output_limb_58_col337
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_59_col338]: [QM31; 1] =
            (*partial_ec_mul_output_limb_59_col338
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_60_col339]: [QM31; 1] =
            (*partial_ec_mul_output_limb_60_col339
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_61_col340]: [QM31; 1] =
            (*partial_ec_mul_output_limb_61_col340
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_62_col341]: [QM31; 1] =
            (*partial_ec_mul_output_limb_62_col341
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_63_col342]: [QM31; 1] =
            (*partial_ec_mul_output_limb_63_col342
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_64_col343]: [QM31; 1] =
            (*partial_ec_mul_output_limb_64_col343
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_65_col344]: [QM31; 1] =
            (*partial_ec_mul_output_limb_65_col344
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_66_col345]: [QM31; 1] =
            (*partial_ec_mul_output_limb_66_col345
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_67_col346]: [QM31; 1] =
            (*partial_ec_mul_output_limb_67_col346
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_68_col347]: [QM31; 1] =
            (*partial_ec_mul_output_limb_68_col347
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_69_col348]: [QM31; 1] =
            (*partial_ec_mul_output_limb_69_col348
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_70_col349]: [QM31; 1] =
            (*partial_ec_mul_output_limb_70_col349
            .try_into()
            .unwrap())
            .unbox();
        let [pedersen_result_id_col350]: [QM31; 1] = (*pedersen_result_id_col350
            .try_into()
            .unwrap())
            .unbox();

        core::internal::revoke_ap_tracking();

        let instance_addr_tmp_d00c6_0: QM31 = ((seq * qm31_const::<3, 0, 0, 0>())
            + pedersen_builtin_segment_start);

        let output: [QM31; 1] = read_split_evaluate(
            [instance_addr_tmp_d00c6_0],
            value_limb_0_col0,
            value_limb_1_col1,
            value_limb_2_col2,
            value_limb_3_col3,
            value_limb_4_col4,
            value_limb_5_col5,
            value_limb_6_col6,
            value_limb_7_col7,
            value_limb_8_col8,
            value_limb_9_col9,
            value_limb_10_col10,
            value_limb_11_col11,
            value_limb_12_col12,
            value_limb_13_col13,
            value_limb_14_col14,
            value_limb_15_col15,
            value_limb_16_col16,
            value_limb_17_col17,
            value_limb_18_col18,
            value_limb_19_col19,
            value_limb_20_col20,
            value_limb_21_col21,
            value_limb_22_col22,
            value_limb_23_col23,
            value_limb_24_col24,
            value_limb_25_col25,
            value_limb_26_col26,
            ms_limb_low_col27,
            ms_limb_high_col28,
            pedersen_a_id_col29,
            self.range_check_5_4_lookup_elements,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref range_check_5_4_sum_0,
            ref memory_address_to_id_sum_1,
            ref memory_id_to_big_sum_2,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let [read_split_output_tmp_d00c6_6_original_limb_27] = output;

        let output: [QM31; 1] = read_split_evaluate(
            [(instance_addr_tmp_d00c6_0 + qm31_const::<1, 0, 0, 0>())],
            value_limb_0_col30,
            value_limb_1_col31,
            value_limb_2_col32,
            value_limb_3_col33,
            value_limb_4_col34,
            value_limb_5_col35,
            value_limb_6_col36,
            value_limb_7_col37,
            value_limb_8_col38,
            value_limb_9_col39,
            value_limb_10_col40,
            value_limb_11_col41,
            value_limb_12_col42,
            value_limb_13_col43,
            value_limb_14_col44,
            value_limb_15_col45,
            value_limb_16_col46,
            value_limb_17_col47,
            value_limb_18_col48,
            value_limb_19_col49,
            value_limb_20_col50,
            value_limb_21_col51,
            value_limb_22_col52,
            value_limb_23_col53,
            value_limb_24_col54,
            value_limb_25_col55,
            value_limb_26_col56,
            ms_limb_low_col57,
            ms_limb_high_col58,
            pedersen_b_id_col59,
            self.range_check_5_4_lookup_elements,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref range_check_5_4_sum_3,
            ref memory_address_to_id_sum_4,
            ref memory_id_to_big_sum_5,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let [read_split_output_tmp_d00c6_12_original_limb_27] = output;

        verify_reduced_252_evaluate(
            [
                value_limb_0_col0, value_limb_1_col1, value_limb_2_col2, value_limb_3_col3,
                value_limb_4_col4, value_limb_5_col5, value_limb_6_col6, value_limb_7_col7,
                value_limb_8_col8, value_limb_9_col9, value_limb_10_col10, value_limb_11_col11,
                value_limb_12_col12, value_limb_13_col13, value_limb_14_col14, value_limb_15_col15,
                value_limb_16_col16, value_limb_17_col17, value_limb_18_col18, value_limb_19_col19,
                value_limb_20_col20, value_limb_21_col21, value_limb_22_col22, value_limb_23_col23,
                value_limb_24_col24, value_limb_25_col25, value_limb_26_col26,
                read_split_output_tmp_d00c6_6_original_limb_27,
            ],
            ms_limb_is_max_col60,
            ms_and_mid_limbs_are_max_col61,
            rc_input_col62,
            self.range_check_8_lookup_elements,
            ref range_check_8_sum_6,
            ref range_check_8_sum_7,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        verify_reduced_252_evaluate(
            [
                value_limb_0_col30, value_limb_1_col31, value_limb_2_col32, value_limb_3_col33,
                value_limb_4_col34, value_limb_5_col35, value_limb_6_col36, value_limb_7_col37,
                value_limb_8_col38, value_limb_9_col39, value_limb_10_col40, value_limb_11_col41,
                value_limb_12_col42, value_limb_13_col43, value_limb_14_col44, value_limb_15_col45,
                value_limb_16_col46, value_limb_17_col47, value_limb_18_col48, value_limb_19_col49,
                value_limb_20_col50, value_limb_21_col51, value_limb_22_col52, value_limb_23_col53,
                value_limb_24_col54, value_limb_25_col55, value_limb_26_col56,
                read_split_output_tmp_d00c6_12_original_limb_27,
            ],
            ms_limb_is_max_col63,
            ms_and_mid_limbs_are_max_col64,
            rc_input_col65,
            self.range_check_8_lookup_elements,
            ref range_check_8_sum_8,
            ref range_check_8_sum_9,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let partial_ec_mul_chain_tmp_tmp_d00c6_17: QM31 = (seq * qm31_const::<4, 0, 0, 0>());

        partial_ec_mul_sum_10 = self
            .partial_ec_mul_lookup_elements
            .combine_qm31(
                [
                    partial_ec_mul_chain_tmp_tmp_d00c6_17, qm31_const::<0, 0, 0, 0>(),
                    qm31_const::<0, 0, 0, 0>(),
                    (value_limb_0_col0 + (value_limb_1_col1 * qm31_const::<512, 0, 0, 0>())),
                    (value_limb_2_col2 + (value_limb_3_col3 * qm31_const::<512, 0, 0, 0>())),
                    (value_limb_4_col4 + (value_limb_5_col5 * qm31_const::<512, 0, 0, 0>())),
                    (value_limb_6_col6 + (value_limb_7_col7 * qm31_const::<512, 0, 0, 0>())),
                    (value_limb_8_col8 + (value_limb_9_col9 * qm31_const::<512, 0, 0, 0>())),
                    (value_limb_10_col10 + (value_limb_11_col11 * qm31_const::<512, 0, 0, 0>())),
                    (value_limb_12_col12 + (value_limb_13_col13 * qm31_const::<512, 0, 0, 0>())),
                    (value_limb_14_col14 + (value_limb_15_col15 * qm31_const::<512, 0, 0, 0>())),
                    (value_limb_16_col16 + (value_limb_17_col17 * qm31_const::<512, 0, 0, 0>())),
                    (value_limb_18_col18 + (value_limb_19_col19 * qm31_const::<512, 0, 0, 0>())),
                    (value_limb_20_col20 + (value_limb_21_col21 * qm31_const::<512, 0, 0, 0>())),
                    (value_limb_22_col22 + (value_limb_23_col23 * qm31_const::<512, 0, 0, 0>())),
                    (value_limb_24_col24 + (value_limb_25_col25 * qm31_const::<512, 0, 0, 0>())),
                    (value_limb_26_col26 + (ms_limb_low_col27 * qm31_const::<512, 0, 0, 0>())),
                    qm31_const::<435, 0, 0, 0>(), qm31_const::<50, 0, 0, 0>(),
                    qm31_const::<508, 0, 0, 0>(), qm31_const::<83, 0, 0, 0>(),
                    qm31_const::<221, 0, 0, 0>(), qm31_const::<281, 0, 0, 0>(),
                    qm31_const::<377, 0, 0, 0>(), qm31_const::<383, 0, 0, 0>(),
                    qm31_const::<212, 0, 0, 0>(), qm31_const::<264, 0, 0, 0>(),
                    qm31_const::<301, 0, 0, 0>(), qm31_const::<458, 0, 0, 0>(),
                    qm31_const::<130, 0, 0, 0>(), qm31_const::<102, 0, 0, 0>(),
                    qm31_const::<385, 0, 0, 0>(), qm31_const::<269, 0, 0, 0>(),
                    qm31_const::<145, 0, 0, 0>(), qm31_const::<276, 0, 0, 0>(),
                    qm31_const::<483, 0, 0, 0>(), qm31_const::<226, 0, 0, 0>(),
                    qm31_const::<422, 0, 0, 0>(), qm31_const::<253, 0, 0, 0>(),
                    qm31_const::<308, 0, 0, 0>(), qm31_const::<125, 0, 0, 0>(),
                    qm31_const::<472, 0, 0, 0>(), qm31_const::<301, 0, 0, 0>(),
                    qm31_const::<227, 0, 0, 0>(), qm31_const::<27, 0, 0, 0>(),
                    qm31_const::<92, 0, 0, 0>(), qm31_const::<321, 0, 0, 0>(),
                    qm31_const::<252, 0, 0, 0>(), qm31_const::<259, 0, 0, 0>(),
                    qm31_const::<252, 0, 0, 0>(), qm31_const::<413, 0, 0, 0>(),
                    qm31_const::<228, 0, 0, 0>(), qm31_const::<31, 0, 0, 0>(),
                    qm31_const::<24, 0, 0, 0>(), qm31_const::<118, 0, 0, 0>(),
                    qm31_const::<301, 0, 0, 0>(), qm31_const::<202, 0, 0, 0>(),
                    qm31_const::<15, 0, 0, 0>(), qm31_const::<464, 0, 0, 0>(),
                    qm31_const::<334, 0, 0, 0>(), qm31_const::<212, 0, 0, 0>(),
                    qm31_const::<471, 0, 0, 0>(), qm31_const::<461, 0, 0, 0>(),
                    qm31_const::<419, 0, 0, 0>(), qm31_const::<354, 0, 0, 0>(),
                    qm31_const::<96, 0, 0, 0>(), qm31_const::<213, 0, 0, 0>(),
                    qm31_const::<319, 0, 0, 0>(), qm31_const::<191, 0, 0, 0>(),
                    qm31_const::<251, 0, 0, 0>(), qm31_const::<330, 0, 0, 0>(),
                    qm31_const::<15, 0, 0, 0>(), qm31_const::<222, 0, 0, 0>(),
                ],
            );

        partial_ec_mul_sum_11 = self
            .partial_ec_mul_lookup_elements
            .combine_qm31(
                [
                    partial_ec_mul_chain_tmp_tmp_d00c6_17, qm31_const::<14, 0, 0, 0>(),
                    partial_ec_mul_output_limb_0_col66, partial_ec_mul_output_limb_1_col67,
                    partial_ec_mul_output_limb_2_col68, partial_ec_mul_output_limb_3_col69,
                    partial_ec_mul_output_limb_4_col70, partial_ec_mul_output_limb_5_col71,
                    partial_ec_mul_output_limb_6_col72, partial_ec_mul_output_limb_7_col73,
                    partial_ec_mul_output_limb_8_col74, partial_ec_mul_output_limb_9_col75,
                    partial_ec_mul_output_limb_10_col76, partial_ec_mul_output_limb_11_col77,
                    partial_ec_mul_output_limb_12_col78, partial_ec_mul_output_limb_13_col79,
                    partial_ec_mul_output_limb_14_col80, partial_ec_mul_output_limb_15_col81,
                    partial_ec_mul_output_limb_16_col82, partial_ec_mul_output_limb_17_col83,
                    partial_ec_mul_output_limb_18_col84, partial_ec_mul_output_limb_19_col85,
                    partial_ec_mul_output_limb_20_col86, partial_ec_mul_output_limb_21_col87,
                    partial_ec_mul_output_limb_22_col88, partial_ec_mul_output_limb_23_col89,
                    partial_ec_mul_output_limb_24_col90, partial_ec_mul_output_limb_25_col91,
                    partial_ec_mul_output_limb_26_col92, partial_ec_mul_output_limb_27_col93,
                    partial_ec_mul_output_limb_28_col94, partial_ec_mul_output_limb_29_col95,
                    partial_ec_mul_output_limb_30_col96, partial_ec_mul_output_limb_31_col97,
                    partial_ec_mul_output_limb_32_col98, partial_ec_mul_output_limb_33_col99,
                    partial_ec_mul_output_limb_34_col100, partial_ec_mul_output_limb_35_col101,
                    partial_ec_mul_output_limb_36_col102, partial_ec_mul_output_limb_37_col103,
                    partial_ec_mul_output_limb_38_col104, partial_ec_mul_output_limb_39_col105,
                    partial_ec_mul_output_limb_40_col106, partial_ec_mul_output_limb_41_col107,
                    partial_ec_mul_output_limb_42_col108, partial_ec_mul_output_limb_43_col109,
                    partial_ec_mul_output_limb_44_col110, partial_ec_mul_output_limb_45_col111,
                    partial_ec_mul_output_limb_46_col112, partial_ec_mul_output_limb_47_col113,
                    partial_ec_mul_output_limb_48_col114, partial_ec_mul_output_limb_49_col115,
                    partial_ec_mul_output_limb_50_col116, partial_ec_mul_output_limb_51_col117,
                    partial_ec_mul_output_limb_52_col118, partial_ec_mul_output_limb_53_col119,
                    partial_ec_mul_output_limb_54_col120, partial_ec_mul_output_limb_55_col121,
                    partial_ec_mul_output_limb_56_col122, partial_ec_mul_output_limb_57_col123,
                    partial_ec_mul_output_limb_58_col124, partial_ec_mul_output_limb_59_col125,
                    partial_ec_mul_output_limb_60_col126, partial_ec_mul_output_limb_61_col127,
                    partial_ec_mul_output_limb_62_col128, partial_ec_mul_output_limb_63_col129,
                    partial_ec_mul_output_limb_64_col130, partial_ec_mul_output_limb_65_col131,
                    partial_ec_mul_output_limb_66_col132, partial_ec_mul_output_limb_67_col133,
                    partial_ec_mul_output_limb_68_col134, partial_ec_mul_output_limb_69_col135,
                    partial_ec_mul_output_limb_70_col136,
                ],
            );
        let partial_ec_mul_chain_id_tmp_d00c6_32: QM31 = (partial_ec_mul_chain_tmp_tmp_d00c6_17
            + qm31_const::<1, 0, 0, 0>());

        partial_ec_mul_sum_12 = self
            .partial_ec_mul_lookup_elements
            .combine_qm31(
                [
                    partial_ec_mul_chain_id_tmp_d00c6_32, qm31_const::<0, 0, 0, 0>(),
                    qm31_const::<3670016, 0, 0, 0>(), ms_limb_high_col28,
                    qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(),
                    qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(),
                    qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(),
                    qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(),
                    qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(),
                    qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(),
                    qm31_const::<0, 0, 0, 0>(), partial_ec_mul_output_limb_15_col81,
                    partial_ec_mul_output_limb_16_col82, partial_ec_mul_output_limb_17_col83,
                    partial_ec_mul_output_limb_18_col84, partial_ec_mul_output_limb_19_col85,
                    partial_ec_mul_output_limb_20_col86, partial_ec_mul_output_limb_21_col87,
                    partial_ec_mul_output_limb_22_col88, partial_ec_mul_output_limb_23_col89,
                    partial_ec_mul_output_limb_24_col90, partial_ec_mul_output_limb_25_col91,
                    partial_ec_mul_output_limb_26_col92, partial_ec_mul_output_limb_27_col93,
                    partial_ec_mul_output_limb_28_col94, partial_ec_mul_output_limb_29_col95,
                    partial_ec_mul_output_limb_30_col96, partial_ec_mul_output_limb_31_col97,
                    partial_ec_mul_output_limb_32_col98, partial_ec_mul_output_limb_33_col99,
                    partial_ec_mul_output_limb_34_col100, partial_ec_mul_output_limb_35_col101,
                    partial_ec_mul_output_limb_36_col102, partial_ec_mul_output_limb_37_col103,
                    partial_ec_mul_output_limb_38_col104, partial_ec_mul_output_limb_39_col105,
                    partial_ec_mul_output_limb_40_col106, partial_ec_mul_output_limb_41_col107,
                    partial_ec_mul_output_limb_42_col108, partial_ec_mul_output_limb_43_col109,
                    partial_ec_mul_output_limb_44_col110, partial_ec_mul_output_limb_45_col111,
                    partial_ec_mul_output_limb_46_col112, partial_ec_mul_output_limb_47_col113,
                    partial_ec_mul_output_limb_48_col114, partial_ec_mul_output_limb_49_col115,
                    partial_ec_mul_output_limb_50_col116, partial_ec_mul_output_limb_51_col117,
                    partial_ec_mul_output_limb_52_col118, partial_ec_mul_output_limb_53_col119,
                    partial_ec_mul_output_limb_54_col120, partial_ec_mul_output_limb_55_col121,
                    partial_ec_mul_output_limb_56_col122, partial_ec_mul_output_limb_57_col123,
                    partial_ec_mul_output_limb_58_col124, partial_ec_mul_output_limb_59_col125,
                    partial_ec_mul_output_limb_60_col126, partial_ec_mul_output_limb_61_col127,
                    partial_ec_mul_output_limb_62_col128, partial_ec_mul_output_limb_63_col129,
                    partial_ec_mul_output_limb_64_col130, partial_ec_mul_output_limb_65_col131,
                    partial_ec_mul_output_limb_66_col132, partial_ec_mul_output_limb_67_col133,
                    partial_ec_mul_output_limb_68_col134, partial_ec_mul_output_limb_69_col135,
                    partial_ec_mul_output_limb_70_col136,
                ],
            );

        partial_ec_mul_sum_13 = self
            .partial_ec_mul_lookup_elements
            .combine_qm31(
                [
                    partial_ec_mul_chain_id_tmp_d00c6_32, qm31_const::<1, 0, 0, 0>(),
                    partial_ec_mul_output_limb_0_col137, partial_ec_mul_output_limb_1_col138,
                    partial_ec_mul_output_limb_2_col139, partial_ec_mul_output_limb_3_col140,
                    partial_ec_mul_output_limb_4_col141, partial_ec_mul_output_limb_5_col142,
                    partial_ec_mul_output_limb_6_col143, partial_ec_mul_output_limb_7_col144,
                    partial_ec_mul_output_limb_8_col145, partial_ec_mul_output_limb_9_col146,
                    partial_ec_mul_output_limb_10_col147, partial_ec_mul_output_limb_11_col148,
                    partial_ec_mul_output_limb_12_col149, partial_ec_mul_output_limb_13_col150,
                    partial_ec_mul_output_limb_14_col151, partial_ec_mul_output_limb_15_col152,
                    partial_ec_mul_output_limb_16_col153, partial_ec_mul_output_limb_17_col154,
                    partial_ec_mul_output_limb_18_col155, partial_ec_mul_output_limb_19_col156,
                    partial_ec_mul_output_limb_20_col157, partial_ec_mul_output_limb_21_col158,
                    partial_ec_mul_output_limb_22_col159, partial_ec_mul_output_limb_23_col160,
                    partial_ec_mul_output_limb_24_col161, partial_ec_mul_output_limb_25_col162,
                    partial_ec_mul_output_limb_26_col163, partial_ec_mul_output_limb_27_col164,
                    partial_ec_mul_output_limb_28_col165, partial_ec_mul_output_limb_29_col166,
                    partial_ec_mul_output_limb_30_col167, partial_ec_mul_output_limb_31_col168,
                    partial_ec_mul_output_limb_32_col169, partial_ec_mul_output_limb_33_col170,
                    partial_ec_mul_output_limb_34_col171, partial_ec_mul_output_limb_35_col172,
                    partial_ec_mul_output_limb_36_col173, partial_ec_mul_output_limb_37_col174,
                    partial_ec_mul_output_limb_38_col175, partial_ec_mul_output_limb_39_col176,
                    partial_ec_mul_output_limb_40_col177, partial_ec_mul_output_limb_41_col178,
                    partial_ec_mul_output_limb_42_col179, partial_ec_mul_output_limb_43_col180,
                    partial_ec_mul_output_limb_44_col181, partial_ec_mul_output_limb_45_col182,
                    partial_ec_mul_output_limb_46_col183, partial_ec_mul_output_limb_47_col184,
                    partial_ec_mul_output_limb_48_col185, partial_ec_mul_output_limb_49_col186,
                    partial_ec_mul_output_limb_50_col187, partial_ec_mul_output_limb_51_col188,
                    partial_ec_mul_output_limb_52_col189, partial_ec_mul_output_limb_53_col190,
                    partial_ec_mul_output_limb_54_col191, partial_ec_mul_output_limb_55_col192,
                    partial_ec_mul_output_limb_56_col193, partial_ec_mul_output_limb_57_col194,
                    partial_ec_mul_output_limb_58_col195, partial_ec_mul_output_limb_59_col196,
                    partial_ec_mul_output_limb_60_col197, partial_ec_mul_output_limb_61_col198,
                    partial_ec_mul_output_limb_62_col199, partial_ec_mul_output_limb_63_col200,
                    partial_ec_mul_output_limb_64_col201, partial_ec_mul_output_limb_65_col202,
                    partial_ec_mul_output_limb_66_col203, partial_ec_mul_output_limb_67_col204,
                    partial_ec_mul_output_limb_68_col205, partial_ec_mul_output_limb_69_col206,
                    partial_ec_mul_output_limb_70_col207,
                ],
            );
        let partial_ec_mul_chain_id_tmp_d00c6_34: QM31 = (partial_ec_mul_chain_tmp_tmp_d00c6_17
            + qm31_const::<2, 0, 0, 0>());

        partial_ec_mul_sum_14 = self
            .partial_ec_mul_lookup_elements
            .combine_qm31(
                [
                    partial_ec_mul_chain_id_tmp_d00c6_34, qm31_const::<0, 0, 0, 0>(),
                    qm31_const::<3670032, 0, 0, 0>(),
                    (value_limb_0_col30 + (value_limb_1_col31 * qm31_const::<512, 0, 0, 0>())),
                    (value_limb_2_col32 + (value_limb_3_col33 * qm31_const::<512, 0, 0, 0>())),
                    (value_limb_4_col34 + (value_limb_5_col35 * qm31_const::<512, 0, 0, 0>())),
                    (value_limb_6_col36 + (value_limb_7_col37 * qm31_const::<512, 0, 0, 0>())),
                    (value_limb_8_col38 + (value_limb_9_col39 * qm31_const::<512, 0, 0, 0>())),
                    (value_limb_10_col40 + (value_limb_11_col41 * qm31_const::<512, 0, 0, 0>())),
                    (value_limb_12_col42 + (value_limb_13_col43 * qm31_const::<512, 0, 0, 0>())),
                    (value_limb_14_col44 + (value_limb_15_col45 * qm31_const::<512, 0, 0, 0>())),
                    (value_limb_16_col46 + (value_limb_17_col47 * qm31_const::<512, 0, 0, 0>())),
                    (value_limb_18_col48 + (value_limb_19_col49 * qm31_const::<512, 0, 0, 0>())),
                    (value_limb_20_col50 + (value_limb_21_col51 * qm31_const::<512, 0, 0, 0>())),
                    (value_limb_22_col52 + (value_limb_23_col53 * qm31_const::<512, 0, 0, 0>())),
                    (value_limb_24_col54 + (value_limb_25_col55 * qm31_const::<512, 0, 0, 0>())),
                    (value_limb_26_col56 + (ms_limb_low_col57 * qm31_const::<512, 0, 0, 0>())),
                    partial_ec_mul_output_limb_15_col152, partial_ec_mul_output_limb_16_col153,
                    partial_ec_mul_output_limb_17_col154, partial_ec_mul_output_limb_18_col155,
                    partial_ec_mul_output_limb_19_col156, partial_ec_mul_output_limb_20_col157,
                    partial_ec_mul_output_limb_21_col158, partial_ec_mul_output_limb_22_col159,
                    partial_ec_mul_output_limb_23_col160, partial_ec_mul_output_limb_24_col161,
                    partial_ec_mul_output_limb_25_col162, partial_ec_mul_output_limb_26_col163,
                    partial_ec_mul_output_limb_27_col164, partial_ec_mul_output_limb_28_col165,
                    partial_ec_mul_output_limb_29_col166, partial_ec_mul_output_limb_30_col167,
                    partial_ec_mul_output_limb_31_col168, partial_ec_mul_output_limb_32_col169,
                    partial_ec_mul_output_limb_33_col170, partial_ec_mul_output_limb_34_col171,
                    partial_ec_mul_output_limb_35_col172, partial_ec_mul_output_limb_36_col173,
                    partial_ec_mul_output_limb_37_col174, partial_ec_mul_output_limb_38_col175,
                    partial_ec_mul_output_limb_39_col176, partial_ec_mul_output_limb_40_col177,
                    partial_ec_mul_output_limb_41_col178, partial_ec_mul_output_limb_42_col179,
                    partial_ec_mul_output_limb_43_col180, partial_ec_mul_output_limb_44_col181,
                    partial_ec_mul_output_limb_45_col182, partial_ec_mul_output_limb_46_col183,
                    partial_ec_mul_output_limb_47_col184, partial_ec_mul_output_limb_48_col185,
                    partial_ec_mul_output_limb_49_col186, partial_ec_mul_output_limb_50_col187,
                    partial_ec_mul_output_limb_51_col188, partial_ec_mul_output_limb_52_col189,
                    partial_ec_mul_output_limb_53_col190, partial_ec_mul_output_limb_54_col191,
                    partial_ec_mul_output_limb_55_col192, partial_ec_mul_output_limb_56_col193,
                    partial_ec_mul_output_limb_57_col194, partial_ec_mul_output_limb_58_col195,
                    partial_ec_mul_output_limb_59_col196, partial_ec_mul_output_limb_60_col197,
                    partial_ec_mul_output_limb_61_col198, partial_ec_mul_output_limb_62_col199,
                    partial_ec_mul_output_limb_63_col200, partial_ec_mul_output_limb_64_col201,
                    partial_ec_mul_output_limb_65_col202, partial_ec_mul_output_limb_66_col203,
                    partial_ec_mul_output_limb_67_col204, partial_ec_mul_output_limb_68_col205,
                    partial_ec_mul_output_limb_69_col206, partial_ec_mul_output_limb_70_col207,
                ],
            );

        partial_ec_mul_sum_15 = self
            .partial_ec_mul_lookup_elements
            .combine_qm31(
                [
                    partial_ec_mul_chain_id_tmp_d00c6_34, qm31_const::<14, 0, 0, 0>(),
                    partial_ec_mul_output_limb_0_col208, partial_ec_mul_output_limb_1_col209,
                    partial_ec_mul_output_limb_2_col210, partial_ec_mul_output_limb_3_col211,
                    partial_ec_mul_output_limb_4_col212, partial_ec_mul_output_limb_5_col213,
                    partial_ec_mul_output_limb_6_col214, partial_ec_mul_output_limb_7_col215,
                    partial_ec_mul_output_limb_8_col216, partial_ec_mul_output_limb_9_col217,
                    partial_ec_mul_output_limb_10_col218, partial_ec_mul_output_limb_11_col219,
                    partial_ec_mul_output_limb_12_col220, partial_ec_mul_output_limb_13_col221,
                    partial_ec_mul_output_limb_14_col222, partial_ec_mul_output_limb_15_col223,
                    partial_ec_mul_output_limb_16_col224, partial_ec_mul_output_limb_17_col225,
                    partial_ec_mul_output_limb_18_col226, partial_ec_mul_output_limb_19_col227,
                    partial_ec_mul_output_limb_20_col228, partial_ec_mul_output_limb_21_col229,
                    partial_ec_mul_output_limb_22_col230, partial_ec_mul_output_limb_23_col231,
                    partial_ec_mul_output_limb_24_col232, partial_ec_mul_output_limb_25_col233,
                    partial_ec_mul_output_limb_26_col234, partial_ec_mul_output_limb_27_col235,
                    partial_ec_mul_output_limb_28_col236, partial_ec_mul_output_limb_29_col237,
                    partial_ec_mul_output_limb_30_col238, partial_ec_mul_output_limb_31_col239,
                    partial_ec_mul_output_limb_32_col240, partial_ec_mul_output_limb_33_col241,
                    partial_ec_mul_output_limb_34_col242, partial_ec_mul_output_limb_35_col243,
                    partial_ec_mul_output_limb_36_col244, partial_ec_mul_output_limb_37_col245,
                    partial_ec_mul_output_limb_38_col246, partial_ec_mul_output_limb_39_col247,
                    partial_ec_mul_output_limb_40_col248, partial_ec_mul_output_limb_41_col249,
                    partial_ec_mul_output_limb_42_col250, partial_ec_mul_output_limb_43_col251,
                    partial_ec_mul_output_limb_44_col252, partial_ec_mul_output_limb_45_col253,
                    partial_ec_mul_output_limb_46_col254, partial_ec_mul_output_limb_47_col255,
                    partial_ec_mul_output_limb_48_col256, partial_ec_mul_output_limb_49_col257,
                    partial_ec_mul_output_limb_50_col258, partial_ec_mul_output_limb_51_col259,
                    partial_ec_mul_output_limb_52_col260, partial_ec_mul_output_limb_53_col261,
                    partial_ec_mul_output_limb_54_col262, partial_ec_mul_output_limb_55_col263,
                    partial_ec_mul_output_limb_56_col264, partial_ec_mul_output_limb_57_col265,
                    partial_ec_mul_output_limb_58_col266, partial_ec_mul_output_limb_59_col267,
                    partial_ec_mul_output_limb_60_col268, partial_ec_mul_output_limb_61_col269,
                    partial_ec_mul_output_limb_62_col270, partial_ec_mul_output_limb_63_col271,
                    partial_ec_mul_output_limb_64_col272, partial_ec_mul_output_limb_65_col273,
                    partial_ec_mul_output_limb_66_col274, partial_ec_mul_output_limb_67_col275,
                    partial_ec_mul_output_limb_68_col276, partial_ec_mul_output_limb_69_col277,
                    partial_ec_mul_output_limb_70_col278,
                ],
            );
        let partial_ec_mul_chain_id_tmp_d00c6_49: QM31 = (partial_ec_mul_chain_tmp_tmp_d00c6_17
            + qm31_const::<3, 0, 0, 0>());

        partial_ec_mul_sum_16 = self
            .partial_ec_mul_lookup_elements
            .combine_qm31(
                [
                    partial_ec_mul_chain_id_tmp_d00c6_49, qm31_const::<0, 0, 0, 0>(),
                    qm31_const::<7340048, 0, 0, 0>(), ms_limb_high_col58,
                    qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(),
                    qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(),
                    qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(),
                    qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(),
                    qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(),
                    qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(),
                    qm31_const::<0, 0, 0, 0>(), partial_ec_mul_output_limb_15_col223,
                    partial_ec_mul_output_limb_16_col224, partial_ec_mul_output_limb_17_col225,
                    partial_ec_mul_output_limb_18_col226, partial_ec_mul_output_limb_19_col227,
                    partial_ec_mul_output_limb_20_col228, partial_ec_mul_output_limb_21_col229,
                    partial_ec_mul_output_limb_22_col230, partial_ec_mul_output_limb_23_col231,
                    partial_ec_mul_output_limb_24_col232, partial_ec_mul_output_limb_25_col233,
                    partial_ec_mul_output_limb_26_col234, partial_ec_mul_output_limb_27_col235,
                    partial_ec_mul_output_limb_28_col236, partial_ec_mul_output_limb_29_col237,
                    partial_ec_mul_output_limb_30_col238, partial_ec_mul_output_limb_31_col239,
                    partial_ec_mul_output_limb_32_col240, partial_ec_mul_output_limb_33_col241,
                    partial_ec_mul_output_limb_34_col242, partial_ec_mul_output_limb_35_col243,
                    partial_ec_mul_output_limb_36_col244, partial_ec_mul_output_limb_37_col245,
                    partial_ec_mul_output_limb_38_col246, partial_ec_mul_output_limb_39_col247,
                    partial_ec_mul_output_limb_40_col248, partial_ec_mul_output_limb_41_col249,
                    partial_ec_mul_output_limb_42_col250, partial_ec_mul_output_limb_43_col251,
                    partial_ec_mul_output_limb_44_col252, partial_ec_mul_output_limb_45_col253,
                    partial_ec_mul_output_limb_46_col254, partial_ec_mul_output_limb_47_col255,
                    partial_ec_mul_output_limb_48_col256, partial_ec_mul_output_limb_49_col257,
                    partial_ec_mul_output_limb_50_col258, partial_ec_mul_output_limb_51_col259,
                    partial_ec_mul_output_limb_52_col260, partial_ec_mul_output_limb_53_col261,
                    partial_ec_mul_output_limb_54_col262, partial_ec_mul_output_limb_55_col263,
                    partial_ec_mul_output_limb_56_col264, partial_ec_mul_output_limb_57_col265,
                    partial_ec_mul_output_limb_58_col266, partial_ec_mul_output_limb_59_col267,
                    partial_ec_mul_output_limb_60_col268, partial_ec_mul_output_limb_61_col269,
                    partial_ec_mul_output_limb_62_col270, partial_ec_mul_output_limb_63_col271,
                    partial_ec_mul_output_limb_64_col272, partial_ec_mul_output_limb_65_col273,
                    partial_ec_mul_output_limb_66_col274, partial_ec_mul_output_limb_67_col275,
                    partial_ec_mul_output_limb_68_col276, partial_ec_mul_output_limb_69_col277,
                    partial_ec_mul_output_limb_70_col278,
                ],
            );

        partial_ec_mul_sum_17 = self
            .partial_ec_mul_lookup_elements
            .combine_qm31(
                [
                    partial_ec_mul_chain_id_tmp_d00c6_49, qm31_const::<1, 0, 0, 0>(),
                    partial_ec_mul_output_limb_0_col279, partial_ec_mul_output_limb_1_col280,
                    partial_ec_mul_output_limb_2_col281, partial_ec_mul_output_limb_3_col282,
                    partial_ec_mul_output_limb_4_col283, partial_ec_mul_output_limb_5_col284,
                    partial_ec_mul_output_limb_6_col285, partial_ec_mul_output_limb_7_col286,
                    partial_ec_mul_output_limb_8_col287, partial_ec_mul_output_limb_9_col288,
                    partial_ec_mul_output_limb_10_col289, partial_ec_mul_output_limb_11_col290,
                    partial_ec_mul_output_limb_12_col291, partial_ec_mul_output_limb_13_col292,
                    partial_ec_mul_output_limb_14_col293, partial_ec_mul_output_limb_15_col294,
                    partial_ec_mul_output_limb_16_col295, partial_ec_mul_output_limb_17_col296,
                    partial_ec_mul_output_limb_18_col297, partial_ec_mul_output_limb_19_col298,
                    partial_ec_mul_output_limb_20_col299, partial_ec_mul_output_limb_21_col300,
                    partial_ec_mul_output_limb_22_col301, partial_ec_mul_output_limb_23_col302,
                    partial_ec_mul_output_limb_24_col303, partial_ec_mul_output_limb_25_col304,
                    partial_ec_mul_output_limb_26_col305, partial_ec_mul_output_limb_27_col306,
                    partial_ec_mul_output_limb_28_col307, partial_ec_mul_output_limb_29_col308,
                    partial_ec_mul_output_limb_30_col309, partial_ec_mul_output_limb_31_col310,
                    partial_ec_mul_output_limb_32_col311, partial_ec_mul_output_limb_33_col312,
                    partial_ec_mul_output_limb_34_col313, partial_ec_mul_output_limb_35_col314,
                    partial_ec_mul_output_limb_36_col315, partial_ec_mul_output_limb_37_col316,
                    partial_ec_mul_output_limb_38_col317, partial_ec_mul_output_limb_39_col318,
                    partial_ec_mul_output_limb_40_col319, partial_ec_mul_output_limb_41_col320,
                    partial_ec_mul_output_limb_42_col321, partial_ec_mul_output_limb_43_col322,
                    partial_ec_mul_output_limb_44_col323, partial_ec_mul_output_limb_45_col324,
                    partial_ec_mul_output_limb_46_col325, partial_ec_mul_output_limb_47_col326,
                    partial_ec_mul_output_limb_48_col327, partial_ec_mul_output_limb_49_col328,
                    partial_ec_mul_output_limb_50_col329, partial_ec_mul_output_limb_51_col330,
                    partial_ec_mul_output_limb_52_col331, partial_ec_mul_output_limb_53_col332,
                    partial_ec_mul_output_limb_54_col333, partial_ec_mul_output_limb_55_col334,
                    partial_ec_mul_output_limb_56_col335, partial_ec_mul_output_limb_57_col336,
                    partial_ec_mul_output_limb_58_col337, partial_ec_mul_output_limb_59_col338,
                    partial_ec_mul_output_limb_60_col339, partial_ec_mul_output_limb_61_col340,
                    partial_ec_mul_output_limb_62_col341, partial_ec_mul_output_limb_63_col342,
                    partial_ec_mul_output_limb_64_col343, partial_ec_mul_output_limb_65_col344,
                    partial_ec_mul_output_limb_66_col345, partial_ec_mul_output_limb_67_col346,
                    partial_ec_mul_output_limb_68_col347, partial_ec_mul_output_limb_69_col348,
                    partial_ec_mul_output_limb_70_col349,
                ],
            );

        mem_verify_evaluate(
            [
                (instance_addr_tmp_d00c6_0 + qm31_const::<2, 0, 0, 0>()),
                partial_ec_mul_output_limb_15_col294, partial_ec_mul_output_limb_16_col295,
                partial_ec_mul_output_limb_17_col296, partial_ec_mul_output_limb_18_col297,
                partial_ec_mul_output_limb_19_col298, partial_ec_mul_output_limb_20_col299,
                partial_ec_mul_output_limb_21_col300, partial_ec_mul_output_limb_22_col301,
                partial_ec_mul_output_limb_23_col302, partial_ec_mul_output_limb_24_col303,
                partial_ec_mul_output_limb_25_col304, partial_ec_mul_output_limb_26_col305,
                partial_ec_mul_output_limb_27_col306, partial_ec_mul_output_limb_28_col307,
                partial_ec_mul_output_limb_29_col308, partial_ec_mul_output_limb_30_col309,
                partial_ec_mul_output_limb_31_col310, partial_ec_mul_output_limb_32_col311,
                partial_ec_mul_output_limb_33_col312, partial_ec_mul_output_limb_34_col313,
                partial_ec_mul_output_limb_35_col314, partial_ec_mul_output_limb_36_col315,
                partial_ec_mul_output_limb_37_col316, partial_ec_mul_output_limb_38_col317,
                partial_ec_mul_output_limb_39_col318, partial_ec_mul_output_limb_40_col319,
                partial_ec_mul_output_limb_41_col320, partial_ec_mul_output_limb_42_col321,
            ],
            pedersen_result_id_col350,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref memory_address_to_id_sum_18,
            ref memory_id_to_big_sum_19,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        lookup_constraints(
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
            claimed_sum,
            column_size,
            ref interaction_trace_mask_values,
            range_check_5_4_sum_0,
            memory_address_to_id_sum_1,
            memory_id_to_big_sum_2,
            range_check_5_4_sum_3,
            memory_address_to_id_sum_4,
            memory_id_to_big_sum_5,
            range_check_8_sum_6,
            range_check_8_sum_7,
            range_check_8_sum_8,
            range_check_8_sum_9,
            partial_ec_mul_sum_10,
            partial_ec_mul_sum_11,
            partial_ec_mul_sum_12,
            partial_ec_mul_sum_13,
            partial_ec_mul_sum_14,
            partial_ec_mul_sum_15,
            partial_ec_mul_sum_16,
            partial_ec_mul_sum_17,
            memory_address_to_id_sum_18,
            memory_id_to_big_sum_19,
        );
    }
}


fn lookup_constraints(
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
    claimed_sum: QM31,
    column_size: M31,
    ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
    range_check_5_4_sum_0: QM31,
    memory_address_to_id_sum_1: QM31,
    memory_id_to_big_sum_2: QM31,
    range_check_5_4_sum_3: QM31,
    memory_address_to_id_sum_4: QM31,
    memory_id_to_big_sum_5: QM31,
    range_check_8_sum_6: QM31,
    range_check_8_sum_7: QM31,
    range_check_8_sum_8: QM31,
    range_check_8_sum_9: QM31,
    partial_ec_mul_sum_10: QM31,
    partial_ec_mul_sum_11: QM31,
    partial_ec_mul_sum_12: QM31,
    partial_ec_mul_sum_13: QM31,
    partial_ec_mul_sum_14: QM31,
    partial_ec_mul_sum_15: QM31,
    partial_ec_mul_sum_16: QM31,
    partial_ec_mul_sum_17: QM31,
    memory_address_to_id_sum_18: QM31,
    memory_id_to_big_sum_19: QM31,
) {
    let [
        trace_2_col0,
        trace_2_col1,
        trace_2_col2,
        trace_2_col3,
        trace_2_col4,
        trace_2_col5,
        trace_2_col6,
        trace_2_col7,
        trace_2_col8,
        trace_2_col9,
        trace_2_col10,
        trace_2_col11,
        trace_2_col12,
        trace_2_col13,
        trace_2_col14,
        trace_2_col15,
        trace_2_col16,
        trace_2_col17,
        trace_2_col18,
        trace_2_col19,
        trace_2_col20,
        trace_2_col21,
        trace_2_col22,
        trace_2_col23,
        trace_2_col24,
        trace_2_col25,
        trace_2_col26,
        trace_2_col27,
        trace_2_col28,
        trace_2_col29,
        trace_2_col30,
        trace_2_col31,
        trace_2_col32,
        trace_2_col33,
        trace_2_col34,
        trace_2_col35,
        trace_2_col36,
        trace_2_col37,
        trace_2_col38,
        trace_2_col39,
    ]: [Span<QM31>; 40] =
        (*interaction_trace_mask_values
        .multi_pop_front()
        .unwrap())
        .unbox();

    let [trace_2_col0]: [QM31; 1] = (*trace_2_col0.try_into().unwrap()).unbox();
    let [trace_2_col1]: [QM31; 1] = (*trace_2_col1.try_into().unwrap()).unbox();
    let [trace_2_col2]: [QM31; 1] = (*trace_2_col2.try_into().unwrap()).unbox();
    let [trace_2_col3]: [QM31; 1] = (*trace_2_col3.try_into().unwrap()).unbox();
    let [trace_2_col4]: [QM31; 1] = (*trace_2_col4.try_into().unwrap()).unbox();
    let [trace_2_col5]: [QM31; 1] = (*trace_2_col5.try_into().unwrap()).unbox();
    let [trace_2_col6]: [QM31; 1] = (*trace_2_col6.try_into().unwrap()).unbox();
    let [trace_2_col7]: [QM31; 1] = (*trace_2_col7.try_into().unwrap()).unbox();
    let [trace_2_col8]: [QM31; 1] = (*trace_2_col8.try_into().unwrap()).unbox();
    let [trace_2_col9]: [QM31; 1] = (*trace_2_col9.try_into().unwrap()).unbox();
    let [trace_2_col10]: [QM31; 1] = (*trace_2_col10.try_into().unwrap()).unbox();
    let [trace_2_col11]: [QM31; 1] = (*trace_2_col11.try_into().unwrap()).unbox();
    let [trace_2_col12]: [QM31; 1] = (*trace_2_col12.try_into().unwrap()).unbox();
    let [trace_2_col13]: [QM31; 1] = (*trace_2_col13.try_into().unwrap()).unbox();
    let [trace_2_col14]: [QM31; 1] = (*trace_2_col14.try_into().unwrap()).unbox();
    let [trace_2_col15]: [QM31; 1] = (*trace_2_col15.try_into().unwrap()).unbox();
    let [trace_2_col16]: [QM31; 1] = (*trace_2_col16.try_into().unwrap()).unbox();
    let [trace_2_col17]: [QM31; 1] = (*trace_2_col17.try_into().unwrap()).unbox();
    let [trace_2_col18]: [QM31; 1] = (*trace_2_col18.try_into().unwrap()).unbox();
    let [trace_2_col19]: [QM31; 1] = (*trace_2_col19.try_into().unwrap()).unbox();
    let [trace_2_col20]: [QM31; 1] = (*trace_2_col20.try_into().unwrap()).unbox();
    let [trace_2_col21]: [QM31; 1] = (*trace_2_col21.try_into().unwrap()).unbox();
    let [trace_2_col22]: [QM31; 1] = (*trace_2_col22.try_into().unwrap()).unbox();
    let [trace_2_col23]: [QM31; 1] = (*trace_2_col23.try_into().unwrap()).unbox();
    let [trace_2_col24]: [QM31; 1] = (*trace_2_col24.try_into().unwrap()).unbox();
    let [trace_2_col25]: [QM31; 1] = (*trace_2_col25.try_into().unwrap()).unbox();
    let [trace_2_col26]: [QM31; 1] = (*trace_2_col26.try_into().unwrap()).unbox();
    let [trace_2_col27]: [QM31; 1] = (*trace_2_col27.try_into().unwrap()).unbox();
    let [trace_2_col28]: [QM31; 1] = (*trace_2_col28.try_into().unwrap()).unbox();
    let [trace_2_col29]: [QM31; 1] = (*trace_2_col29.try_into().unwrap()).unbox();
    let [trace_2_col30]: [QM31; 1] = (*trace_2_col30.try_into().unwrap()).unbox();
    let [trace_2_col31]: [QM31; 1] = (*trace_2_col31.try_into().unwrap()).unbox();
    let [trace_2_col32]: [QM31; 1] = (*trace_2_col32.try_into().unwrap()).unbox();
    let [trace_2_col33]: [QM31; 1] = (*trace_2_col33.try_into().unwrap()).unbox();
    let [trace_2_col34]: [QM31; 1] = (*trace_2_col34.try_into().unwrap()).unbox();
    let [trace_2_col35]: [QM31; 1] = (*trace_2_col35.try_into().unwrap()).unbox();
    let [trace_2_col36_neg1, trace_2_col36]: [QM31; 2] = (*trace_2_col36.try_into().unwrap())
        .unbox();
    let [trace_2_col37_neg1, trace_2_col37]: [QM31; 2] = (*trace_2_col37.try_into().unwrap())
        .unbox();
    let [trace_2_col38_neg1, trace_2_col38]: [QM31; 2] = (*trace_2_col38.try_into().unwrap())
        .unbox();
    let [trace_2_col39_neg1, trace_2_col39]: [QM31; 2] = (*trace_2_col39.try_into().unwrap())
        .unbox();

    core::internal::revoke_ap_tracking();

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3],
    ))
        * range_check_5_4_sum_0
        * memory_address_to_id_sum_1)
        - range_check_5_4_sum_0
        - memory_address_to_id_sum_1)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7],
    )
        - QM31Impl::from_partial_evals([trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3]))
        * memory_id_to_big_sum_2
        * range_check_5_4_sum_3)
        - memory_id_to_big_sum_2
        - range_check_5_4_sum_3)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11],
    )
        - QM31Impl::from_partial_evals([trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7]))
        * memory_address_to_id_sum_4
        * memory_id_to_big_sum_5)
        - memory_address_to_id_sum_4
        - memory_id_to_big_sum_5)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15],
    )
        - QM31Impl::from_partial_evals([trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11]))
        * range_check_8_sum_6
        * range_check_8_sum_7)
        - range_check_8_sum_6
        - range_check_8_sum_7)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col16, trace_2_col17, trace_2_col18, trace_2_col19],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15],
        ))
        * range_check_8_sum_8
        * range_check_8_sum_9)
        - range_check_8_sum_8
        - range_check_8_sum_9)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col20, trace_2_col21, trace_2_col22, trace_2_col23],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col16, trace_2_col17, trace_2_col18, trace_2_col19],
        ))
        * partial_ec_mul_sum_10
        * partial_ec_mul_sum_11)
        - partial_ec_mul_sum_10
        + partial_ec_mul_sum_11)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col24, trace_2_col25, trace_2_col26, trace_2_col27],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col20, trace_2_col21, trace_2_col22, trace_2_col23],
        ))
        * partial_ec_mul_sum_12
        * partial_ec_mul_sum_13)
        - partial_ec_mul_sum_12
        + partial_ec_mul_sum_13)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col28, trace_2_col29, trace_2_col30, trace_2_col31],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col24, trace_2_col25, trace_2_col26, trace_2_col27],
        ))
        * partial_ec_mul_sum_14
        * partial_ec_mul_sum_15)
        - partial_ec_mul_sum_14
        + partial_ec_mul_sum_15)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col32, trace_2_col33, trace_2_col34, trace_2_col35],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col28, trace_2_col29, trace_2_col30, trace_2_col31],
        ))
        * partial_ec_mul_sum_16
        * partial_ec_mul_sum_17)
        - partial_ec_mul_sum_16
        + partial_ec_mul_sum_17)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col36, trace_2_col37, trace_2_col38, trace_2_col39],
    )
        - QM31Impl::from_partial_evals([trace_2_col32, trace_2_col33, trace_2_col34, trace_2_col35])
        - QM31Impl::from_partial_evals(
            [trace_2_col36_neg1, trace_2_col37_neg1, trace_2_col38_neg1, trace_2_col39_neg1],
        )
        + (claimed_sum * (column_size.inverse().into())))
        * memory_address_to_id_sum_18
        * memory_id_to_big_sum_19)
        - memory_address_to_id_sum_18
        - memory_id_to_big_sum_19)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
}
