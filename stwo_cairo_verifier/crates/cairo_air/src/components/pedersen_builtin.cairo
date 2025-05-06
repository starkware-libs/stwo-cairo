use core::num::traits::Zero;
use stwo_constraint_framework::{
    PreprocessedColumn, PreprocessedColumnSet, PreprocessedMaskValues, PreprocessedMaskValuesImpl,
};
use stwo_verifier_core::channel::{Channel, ChannelTrait};
use stwo_verifier_core::circle::CirclePoint;
use stwo_verifier_core::fields::Invertible;
use stwo_verifier_core::fields::m31::m31;
use stwo_verifier_core::fields::qm31::{QM31, QM31Serde, QM31_EXTENSION_DEGREE};
use stwo_verifier_core::poly::circle::CanonicCosetImpl;
use stwo_verifier_core::utils::{ArrayImpl, pow2};
use stwo_verifier_core::{ColumnArray, ColumnSpan, TreeArray};
use crate::components::CairoComponent;
use crate::utils::U32Impl;

mod constraints;

#[derive(Drop, Serde, Copy)]
pub struct Claim {
    pub log_size: u32,
    pub pedersen_builtin_segment_start: u32,
}

#[generate_trait]
pub impl ClaimImpl of ClaimTrait {
    fn log_sizes(self: @Claim) -> TreeArray<Span<u32>> {
        let log_size = *self.log_size;
        let preprocessed_log_sizes = array![log_size].span();
        let trace_log_sizes = ArrayImpl::new_repeated(359, log_size).span();
        let interaction_log_sizes = ArrayImpl::new_repeated(QM31_EXTENSION_DEGREE * 10, log_size)
            .span();
        array![preprocessed_log_sizes, trace_log_sizes, interaction_log_sizes]
    }

    fn mix_into(self: @Claim, ref channel: Channel) {
        channel.mix_u64((*self.log_size).into());
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
    pub memory_address_to_id_lookup_elements: crate::MemoryAddressToIdElements,
    pub memory_id_to_big_lookup_elements: crate::MemoryIdToBigElements,
    pub range_check_5_4_lookup_elements: crate::RangeCheck_5_4Elements,
    pub partial_ec_mul_lookup_elements: crate::PartialEcMulElements,
    pub range_check_8_lookup_elements: crate::RangeCheck_8Elements,
}

pub impl CairoComponentImpl of CairoComponent<Component> {
    fn mask_points(
        self: @Component,
        ref preprocessed_column_set: PreprocessedColumnSet,
        ref trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        let log_size = *self.claim.log_size;
        let trace_gen = CanonicCosetImpl::new(log_size).coset.step_size;
        constraints::mask_points(
            ref preprocessed_column_set,
            ref trace_mask_points,
            ref interaction_trace_mask_points,
            point,
            trace_gen,
            log_size,
        );
    }

    fn max_constraint_log_degree_bound(self: @Component) -> u32 {
        *self.claim.log_size + 1
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
        let MemoryAddressToId_z = *self.memory_address_to_id_lookup_elements.z;
        let mut memory_address_to_id_alpha_powers = self
            .memory_address_to_id_lookup_elements
            .alpha_powers
            .span();
        let MemoryAddressToId_alpha0 = *memory_address_to_id_alpha_powers.pop_front().unwrap();
        let MemoryAddressToId_alpha1 = *memory_address_to_id_alpha_powers.pop_front().unwrap();

        let MemoryIdToBig_z = *self.memory_id_to_big_lookup_elements.z;
        let mut memory_id_to_big_alpha_powers = self
            .memory_id_to_big_lookup_elements
            .alpha_powers
            .span();
        let MemoryIdToBig_alpha0 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha1 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha2 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha3 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha4 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha5 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha6 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha7 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha8 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha9 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha10 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha11 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha12 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha13 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha14 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha15 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha16 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha17 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha18 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha19 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha20 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha21 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha22 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha23 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha24 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha25 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha26 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha27 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha28 = *memory_id_to_big_alpha_powers.pop_front().unwrap();

        let RangeCheck_5_4_z = *self.range_check_5_4_lookup_elements.z;
        let mut range_check_5_4_alpha_powers = self
            .range_check_5_4_lookup_elements
            .alpha_powers
            .span();
        let RangeCheck_5_4_alpha0 = *range_check_5_4_alpha_powers.pop_front().unwrap();
        let RangeCheck_5_4_alpha1 = *range_check_5_4_alpha_powers.pop_front().unwrap();

        let PartialEcMul_z = *self.partial_ec_mul_lookup_elements.z;
        let mut PartialEcMul_alpha_powers = self.partial_ec_mul_lookup_elements.alpha_powers.span();
        let PartialEcMul_alpha0 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha1 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha2 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha3 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha4 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha5 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha6 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha7 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha8 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha9 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha10 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha11 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha12 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha13 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha14 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha15 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha16 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha17 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha18 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha19 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha20 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha21 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha22 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha23 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha24 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha25 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha26 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha27 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha28 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha29 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha30 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha31 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha32 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha33 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha34 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha35 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha36 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha37 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha38 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha39 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha40 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha41 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha42 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha43 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha44 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha45 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha46 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha47 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha48 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha49 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha50 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha51 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha52 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha53 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha54 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha55 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha56 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha57 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha58 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha59 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha60 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha61 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha62 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha63 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha64 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha65 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha66 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha67 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha68 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha69 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha70 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha71 = *PartialEcMul_alpha_powers.pop_front().unwrap();
        let PartialEcMul_alpha72 = *PartialEcMul_alpha_powers.pop_front().unwrap();

        let RangeCheck_8_z = *self.range_check_8_lookup_elements.z;
        let mut range_check_8_alpha_powers = self.range_check_8_lookup_elements.alpha_powers.span();
        let RangeCheck_8_alpha0 = *range_check_8_alpha_powers.pop_front().unwrap();

        let log_size = *self.claim.log_size;

        let claimed_sum = *self.interaction_claim.claimed_sum;

        let pedersen_builtin_segment_start = (*self.claim.pedersen_builtin_segment_start)
            .try_into()
            .unwrap();

        let params = constraints::ConstraintParams {
            column_size: pow2(log_size).try_into().unwrap(),
            builtin_segment_start: pedersen_builtin_segment_start,
            RangeCheck_8_alpha0,
            RangeCheck_8_z,
            MemoryAddressToId_alpha0,
            MemoryAddressToId_alpha1,
            MemoryAddressToId_z,
            MemoryIdToBig_alpha0,
            MemoryIdToBig_alpha1,
            MemoryIdToBig_alpha2,
            MemoryIdToBig_alpha3,
            MemoryIdToBig_alpha4,
            MemoryIdToBig_alpha5,
            MemoryIdToBig_alpha6,
            MemoryIdToBig_alpha7,
            MemoryIdToBig_alpha8,
            MemoryIdToBig_alpha9,
            MemoryIdToBig_alpha10,
            MemoryIdToBig_alpha11,
            MemoryIdToBig_alpha12,
            MemoryIdToBig_alpha13,
            MemoryIdToBig_alpha14,
            MemoryIdToBig_alpha15,
            MemoryIdToBig_alpha16,
            MemoryIdToBig_alpha17,
            MemoryIdToBig_alpha18,
            MemoryIdToBig_alpha19,
            MemoryIdToBig_alpha20,
            MemoryIdToBig_alpha21,
            MemoryIdToBig_alpha22,
            MemoryIdToBig_alpha23,
            MemoryIdToBig_alpha24,
            MemoryIdToBig_alpha25,
            MemoryIdToBig_alpha26,
            MemoryIdToBig_alpha27,
            MemoryIdToBig_alpha28,
            MemoryIdToBig_z,
            PartialEcMul_alpha0,
            PartialEcMul_alpha1,
            PartialEcMul_alpha2,
            PartialEcMul_alpha3,
            PartialEcMul_alpha4,
            PartialEcMul_alpha5,
            PartialEcMul_alpha6,
            PartialEcMul_alpha7,
            PartialEcMul_alpha8,
            PartialEcMul_alpha9,
            PartialEcMul_alpha10,
            PartialEcMul_alpha11,
            PartialEcMul_alpha12,
            PartialEcMul_alpha13,
            PartialEcMul_alpha14,
            PartialEcMul_alpha15,
            PartialEcMul_alpha16,
            PartialEcMul_alpha17,
            PartialEcMul_alpha18,
            PartialEcMul_alpha19,
            PartialEcMul_alpha20,
            PartialEcMul_alpha21,
            PartialEcMul_alpha22,
            PartialEcMul_alpha23,
            PartialEcMul_alpha24,
            PartialEcMul_alpha25,
            PartialEcMul_alpha26,
            PartialEcMul_alpha27,
            PartialEcMul_alpha28,
            PartialEcMul_alpha29,
            PartialEcMul_alpha30,
            PartialEcMul_alpha31,
            PartialEcMul_alpha32,
            PartialEcMul_alpha33,
            PartialEcMul_alpha34,
            PartialEcMul_alpha35,
            PartialEcMul_alpha36,
            PartialEcMul_alpha37,
            PartialEcMul_alpha38,
            PartialEcMul_alpha39,
            PartialEcMul_alpha40,
            PartialEcMul_alpha41,
            PartialEcMul_alpha42,
            PartialEcMul_alpha43,
            PartialEcMul_alpha44,
            PartialEcMul_alpha45,
            PartialEcMul_alpha46,
            PartialEcMul_alpha47,
            PartialEcMul_alpha48,
            PartialEcMul_alpha49,
            PartialEcMul_alpha50,
            PartialEcMul_alpha51,
            PartialEcMul_alpha52,
            PartialEcMul_alpha53,
            PartialEcMul_alpha54,
            PartialEcMul_alpha55,
            PartialEcMul_alpha56,
            PartialEcMul_alpha57,
            PartialEcMul_alpha58,
            PartialEcMul_alpha59,
            PartialEcMul_alpha60,
            PartialEcMul_alpha61,
            PartialEcMul_alpha62,
            PartialEcMul_alpha63,
            PartialEcMul_alpha64,
            PartialEcMul_alpha65,
            PartialEcMul_alpha66,
            PartialEcMul_alpha67,
            PartialEcMul_alpha68,
            PartialEcMul_alpha69,
            PartialEcMul_alpha70,
            PartialEcMul_alpha71,
            PartialEcMul_alpha72,
            PartialEcMul_z,
            RangeCheck_5_4_alpha0,
            RangeCheck_5_4_alpha1,
            RangeCheck_5_4_z,
            claimed_sum,
            seq: preprocessed_mask_values.get(PreprocessedColumn::Seq(log_size)),
        };

        let trace_domain = CanonicCosetImpl::new(log_size);
        let vanish_eval = trace_domain.eval_vanishing(point);

        constraints::evaluate_constraints_at_point(
            ref sum,
            ref trace_mask_values,
            ref interaction_trace_mask_values,
            params,
            random_coeff,
            vanish_eval.inverse(),
        );
    }
}

