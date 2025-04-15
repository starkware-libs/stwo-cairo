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
}

#[generate_trait]
pub impl ClaimImpl of ClaimTrait {
    fn log_sizes(self: @Claim) -> TreeArray<Span<u32>> {
        let log_size = *self.log_size;
        let preprocessed_log_sizes = array![log_size].span();
        let trace_log_sizes = ArrayImpl::new_repeated(472, log_size).span();
        let interaction_log_sizes = ArrayImpl::new_repeated(QM31_EXTENSION_DEGREE * 107, log_size)
            .span();
        array![preprocessed_log_sizes, trace_log_sizes, interaction_log_sizes]
    }

    fn mix_into(self: @Claim, ref channel: Channel) {
        channel.mix_u64((*self.log_size).into());
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
    pub partial_ec_mul_lookup_elements: crate::PartialEcMulElements,
    pub pedersen_points_table_lookup_elements: crate::PedersenPointsTableElements,
    pub range_check_19_lookup_elements: crate::RangeCheck19BitElements,
    pub range_check_9_9_lookup_elements: crate::RangeCheck9Bit9BitElements,
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

        let PedersenPointsTable_z = *self.pedersen_points_table_lookup_elements.z;
        let mut PedersenPointsTable_alpha_powers = self
            .pedersen_points_table_lookup_elements
            .alpha_powers
            .span();
        let PedersenPointsTable_alpha0 = *PedersenPointsTable_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha1 = *PedersenPointsTable_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha2 = *PedersenPointsTable_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha3 = *PedersenPointsTable_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha4 = *PedersenPointsTable_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha5 = *PedersenPointsTable_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha6 = *PedersenPointsTable_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha7 = *PedersenPointsTable_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha8 = *PedersenPointsTable_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha9 = *PedersenPointsTable_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha10 = *PedersenPointsTable_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha11 = *PedersenPointsTable_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha12 = *PedersenPointsTable_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha13 = *PedersenPointsTable_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha14 = *PedersenPointsTable_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha15 = *PedersenPointsTable_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha16 = *PedersenPointsTable_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha17 = *PedersenPointsTable_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha18 = *PedersenPointsTable_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha19 = *PedersenPointsTable_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha20 = *PedersenPointsTable_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha21 = *PedersenPointsTable_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha22 = *PedersenPointsTable_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha23 = *PedersenPointsTable_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha24 = *PedersenPointsTable_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha25 = *PedersenPointsTable_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha26 = *PedersenPointsTable_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha27 = *PedersenPointsTable_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha28 = *PedersenPointsTable_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha29 = *PedersenPointsTable_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha30 = *PedersenPointsTable_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha31 = *PedersenPointsTable_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha32 = *PedersenPointsTable_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha33 = *PedersenPointsTable_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha34 = *PedersenPointsTable_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha35 = *PedersenPointsTable_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha36 = *PedersenPointsTable_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha37 = *PedersenPointsTable_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha38 = *PedersenPointsTable_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha39 = *PedersenPointsTable_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha40 = *PedersenPointsTable_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha41 = *PedersenPointsTable_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha42 = *PedersenPointsTable_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha43 = *PedersenPointsTable_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha44 = *PedersenPointsTable_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha45 = *PedersenPointsTable_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha46 = *PedersenPointsTable_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha47 = *PedersenPointsTable_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha48 = *PedersenPointsTable_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha49 = *PedersenPointsTable_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha50 = *PedersenPointsTable_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha51 = *PedersenPointsTable_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha52 = *PedersenPointsTable_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha53 = *PedersenPointsTable_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha54 = *PedersenPointsTable_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha55 = *PedersenPointsTable_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha56 = *PedersenPointsTable_alpha_powers.pop_front().unwrap();

        let RangeCheck_19_z = *self.range_check_19_lookup_elements.z;
        let mut range_check_19_alpha_powers = self
            .range_check_19_lookup_elements
            .alpha_powers
            .span();
        let RangeCheck_19_alpha0 = *range_check_19_alpha_powers.pop_front().unwrap();

        let RangeCheck_9_9_z = *self.range_check_9_9_lookup_elements.z;
        let mut range_check_9_9_alpha_powers = self
            .range_check_9_9_lookup_elements
            .alpha_powers
            .span();
        let RangeCheck_9_9_alpha0 = *range_check_9_9_alpha_powers.pop_front().unwrap();
        let RangeCheck_9_9_alpha1 = *range_check_9_9_alpha_powers.pop_front().unwrap();

        let log_size = *self.claim.log_size;

        let claimed_sum = *self.interaction_claim.claimed_sum;

        let params = constraints::ConstraintParams {
            column_size: m31(pow2(log_size)),
            PartialEcMul_alpha0,
            PartialEcMul_alpha1,
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
            PartialEcMul_alpha2,
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
            PartialEcMul_alpha3,
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
            PartialEcMul_alpha4,
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
            PartialEcMul_alpha5,
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
            PartialEcMul_alpha6,
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
            PartialEcMul_alpha7,
            PartialEcMul_alpha70,
            PartialEcMul_alpha71,
            PartialEcMul_alpha72,
            PartialEcMul_alpha8,
            PartialEcMul_alpha9,
            PartialEcMul_z,
            PedersenPointsTable_alpha0,
            PedersenPointsTable_alpha1,
            PedersenPointsTable_alpha10,
            PedersenPointsTable_alpha11,
            PedersenPointsTable_alpha12,
            PedersenPointsTable_alpha13,
            PedersenPointsTable_alpha14,
            PedersenPointsTable_alpha15,
            PedersenPointsTable_alpha16,
            PedersenPointsTable_alpha17,
            PedersenPointsTable_alpha18,
            PedersenPointsTable_alpha19,
            PedersenPointsTable_alpha2,
            PedersenPointsTable_alpha20,
            PedersenPointsTable_alpha21,
            PedersenPointsTable_alpha22,
            PedersenPointsTable_alpha23,
            PedersenPointsTable_alpha24,
            PedersenPointsTable_alpha25,
            PedersenPointsTable_alpha26,
            PedersenPointsTable_alpha27,
            PedersenPointsTable_alpha28,
            PedersenPointsTable_alpha29,
            PedersenPointsTable_alpha3,
            PedersenPointsTable_alpha30,
            PedersenPointsTable_alpha31,
            PedersenPointsTable_alpha32,
            PedersenPointsTable_alpha33,
            PedersenPointsTable_alpha34,
            PedersenPointsTable_alpha35,
            PedersenPointsTable_alpha36,
            PedersenPointsTable_alpha37,
            PedersenPointsTable_alpha38,
            PedersenPointsTable_alpha39,
            PedersenPointsTable_alpha4,
            PedersenPointsTable_alpha40,
            PedersenPointsTable_alpha41,
            PedersenPointsTable_alpha42,
            PedersenPointsTable_alpha43,
            PedersenPointsTable_alpha44,
            PedersenPointsTable_alpha45,
            PedersenPointsTable_alpha46,
            PedersenPointsTable_alpha47,
            PedersenPointsTable_alpha48,
            PedersenPointsTable_alpha49,
            PedersenPointsTable_alpha5,
            PedersenPointsTable_alpha50,
            PedersenPointsTable_alpha51,
            PedersenPointsTable_alpha52,
            PedersenPointsTable_alpha53,
            PedersenPointsTable_alpha54,
            PedersenPointsTable_alpha55,
            PedersenPointsTable_alpha56,
            PedersenPointsTable_alpha6,
            PedersenPointsTable_alpha7,
            PedersenPointsTable_alpha8,
            PedersenPointsTable_alpha9,
            PedersenPointsTable_z,
            RangeCheck_19_alpha0,
            RangeCheck_19_z,
            RangeCheck_9_9_alpha0,
            RangeCheck_9_9_alpha1,
            RangeCheck_9_9_z,
            claimed_sum,
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

