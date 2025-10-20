// This file was created by the AIR team.

use crate::components::subroutines::mod_utils::mod_utils_evaluate;
use crate::prelude::*;

pub const N_TRACE_COLUMNS: usize = 267;
pub const RELATION_USES_PER_ROW: [(felt252, u32); 2] = [
    ('MemoryAddressToId', 29), ('MemoryIdToBig', 24),
];

#[derive(Drop, Serde, Copy)]
pub struct Claim {
    pub log_size: u32,
    pub add_mod_builtin_segment_start: u32,
}

pub impl ClaimImpl of ClaimTrait<Claim> {
    fn log_sizes(self: @Claim) -> TreeArray<Span<u32>> {
        let log_size = *(self.log_size);
        let preprocessed_log_sizes = array![log_size].span();
        let trace_log_sizes = [log_size; N_TRACE_COLUMNS].span();
        let interaction_log_sizes = [log_size; 108].span();
        array![preprocessed_log_sizes, trace_log_sizes, interaction_log_sizes]
    }

    fn mix_into(self: @Claim, ref channel: Channel) {
        channel.mix_u64((*(self.log_size)).into());
        channel.mix_u64((*self.add_mod_builtin_segment_start).into());
    }

    fn accumulate_relation_uses(self: @Claim, ref relation_uses: RelationUsesDict) {
        accumulate_relation_uses(ref relation_uses, RELATION_USES_PER_ROW.span(), *self.log_size);
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
}

pub impl NewComponentImpl of NewComponent<Component> {
    type Claim = Claim;
    type InteractionClaim = InteractionClaim;

    fn new(
        claim: @Claim,
        interaction_claim: @InteractionClaim,
        interaction_elements: @CairoInteractionElements,
    ) -> Component {
        Component {
            claim: *claim,
            interaction_claim: *interaction_claim,
            memory_address_to_id_lookup_elements: interaction_elements.memory_address_to_id.clone(),
            memory_id_to_big_lookup_elements: interaction_elements.memory_id_to_value.clone(),
        }
    }
}

pub impl CairoComponentImpl of CairoComponent<Component> {
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
        let add_mod_builtin_segment_start: QM31 = (TryInto::<
            u32, M31,
        >::try_into((*(self.claim.add_mod_builtin_segment_start)))
            .unwrap())
            .into();
        let mut memory_address_to_id_sum_0: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_1: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_2: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_3: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_4: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_5: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_6: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_7: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_8: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_9: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_10: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_11: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_12: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_13: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_14: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_15: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_16: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_17: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_18: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_19: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_20: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_21: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_22: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_23: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_24: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_25: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_26: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_27: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_28: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_29: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_30: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_31: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_32: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_33: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_34: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_35: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_36: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_37: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_38: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_39: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_40: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_41: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_42: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_43: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_44: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_45: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_46: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_47: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_48: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_49: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_50: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_51: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_52: QM31 = Zero::zero();
        let seq = preprocessed_mask_values.get(PreprocessedColumn::Seq(*(self.claim.log_size)));

        let [
            is_instance_0_col0,
            p0_id_col1,
            p0_limb_0_col2,
            p0_limb_1_col3,
            p0_limb_2_col4,
            p0_limb_3_col5,
            p0_limb_4_col6,
            p0_limb_5_col7,
            p0_limb_6_col8,
            p0_limb_7_col9,
            p0_limb_8_col10,
            p0_limb_9_col11,
            p0_limb_10_col12,
            p1_id_col13,
            p1_limb_0_col14,
            p1_limb_1_col15,
            p1_limb_2_col16,
            p1_limb_3_col17,
            p1_limb_4_col18,
            p1_limb_5_col19,
            p1_limb_6_col20,
            p1_limb_7_col21,
            p1_limb_8_col22,
            p1_limb_9_col23,
            p1_limb_10_col24,
            p2_id_col25,
            p2_limb_0_col26,
            p2_limb_1_col27,
            p2_limb_2_col28,
            p2_limb_3_col29,
            p2_limb_4_col30,
            p2_limb_5_col31,
            p2_limb_6_col32,
            p2_limb_7_col33,
            p2_limb_8_col34,
            p2_limb_9_col35,
            p2_limb_10_col36,
            p3_id_col37,
            p3_limb_0_col38,
            p3_limb_1_col39,
            p3_limb_2_col40,
            p3_limb_3_col41,
            p3_limb_4_col42,
            p3_limb_5_col43,
            p3_limb_6_col44,
            p3_limb_7_col45,
            p3_limb_8_col46,
            p3_limb_9_col47,
            p3_limb_10_col48,
            values_ptr_id_col49,
            values_ptr_limb_0_col50,
            values_ptr_limb_1_col51,
            values_ptr_limb_2_col52,
            values_ptr_limb_3_col53,
            partial_limb_msb_col54,
            offsets_ptr_id_col55,
            offsets_ptr_limb_0_col56,
            offsets_ptr_limb_1_col57,
            offsets_ptr_limb_2_col58,
            offsets_ptr_limb_3_col59,
            partial_limb_msb_col60,
            offsets_ptr_prev_id_col61,
            offsets_ptr_prev_limb_0_col62,
            offsets_ptr_prev_limb_1_col63,
            offsets_ptr_prev_limb_2_col64,
            offsets_ptr_prev_limb_3_col65,
            partial_limb_msb_col66,
            n_id_col67,
            n_limb_0_col68,
            n_limb_1_col69,
            n_limb_2_col70,
            n_limb_3_col71,
            partial_limb_msb_col72,
            n_prev_id_col73,
            n_prev_limb_0_col74,
            n_prev_limb_1_col75,
            n_prev_limb_2_col76,
            n_prev_limb_3_col77,
            partial_limb_msb_col78,
            values_ptr_prev_id_col79,
            p_prev0_id_col80,
            p_prev1_id_col81,
            p_prev2_id_col82,
            p_prev3_id_col83,
            offsets_a_id_col84,
            msb_col85,
            mid_limbs_set_col86,
            offsets_a_limb_0_col87,
            offsets_a_limb_1_col88,
            offsets_a_limb_2_col89,
            remainder_bits_col90,
            partial_limb_msb_col91,
            offsets_b_id_col92,
            msb_col93,
            mid_limbs_set_col94,
            offsets_b_limb_0_col95,
            offsets_b_limb_1_col96,
            offsets_b_limb_2_col97,
            remainder_bits_col98,
            partial_limb_msb_col99,
            offsets_c_id_col100,
            msb_col101,
            mid_limbs_set_col102,
            offsets_c_limb_0_col103,
            offsets_c_limb_1_col104,
            offsets_c_limb_2_col105,
            remainder_bits_col106,
            partial_limb_msb_col107,
            a0_id_col108,
            a0_limb_0_col109,
            a0_limb_1_col110,
            a0_limb_2_col111,
            a0_limb_3_col112,
            a0_limb_4_col113,
            a0_limb_5_col114,
            a0_limb_6_col115,
            a0_limb_7_col116,
            a0_limb_8_col117,
            a0_limb_9_col118,
            a0_limb_10_col119,
            a1_id_col120,
            a1_limb_0_col121,
            a1_limb_1_col122,
            a1_limb_2_col123,
            a1_limb_3_col124,
            a1_limb_4_col125,
            a1_limb_5_col126,
            a1_limb_6_col127,
            a1_limb_7_col128,
            a1_limb_8_col129,
            a1_limb_9_col130,
            a1_limb_10_col131,
            a2_id_col132,
            a2_limb_0_col133,
            a2_limb_1_col134,
            a2_limb_2_col135,
            a2_limb_3_col136,
            a2_limb_4_col137,
            a2_limb_5_col138,
            a2_limb_6_col139,
            a2_limb_7_col140,
            a2_limb_8_col141,
            a2_limb_9_col142,
            a2_limb_10_col143,
            a3_id_col144,
            a3_limb_0_col145,
            a3_limb_1_col146,
            a3_limb_2_col147,
            a3_limb_3_col148,
            a3_limb_4_col149,
            a3_limb_5_col150,
            a3_limb_6_col151,
            a3_limb_7_col152,
            a3_limb_8_col153,
            a3_limb_9_col154,
            a3_limb_10_col155,
            b0_id_col156,
            b0_limb_0_col157,
            b0_limb_1_col158,
            b0_limb_2_col159,
            b0_limb_3_col160,
            b0_limb_4_col161,
            b0_limb_5_col162,
            b0_limb_6_col163,
            b0_limb_7_col164,
            b0_limb_8_col165,
            b0_limb_9_col166,
            b0_limb_10_col167,
            b1_id_col168,
            b1_limb_0_col169,
            b1_limb_1_col170,
            b1_limb_2_col171,
            b1_limb_3_col172,
            b1_limb_4_col173,
            b1_limb_5_col174,
            b1_limb_6_col175,
            b1_limb_7_col176,
            b1_limb_8_col177,
            b1_limb_9_col178,
            b1_limb_10_col179,
            b2_id_col180,
            b2_limb_0_col181,
            b2_limb_1_col182,
            b2_limb_2_col183,
            b2_limb_3_col184,
            b2_limb_4_col185,
            b2_limb_5_col186,
            b2_limb_6_col187,
            b2_limb_7_col188,
            b2_limb_8_col189,
            b2_limb_9_col190,
            b2_limb_10_col191,
            b3_id_col192,
            b3_limb_0_col193,
            b3_limb_1_col194,
            b3_limb_2_col195,
            b3_limb_3_col196,
            b3_limb_4_col197,
            b3_limb_5_col198,
            b3_limb_6_col199,
            b3_limb_7_col200,
            b3_limb_8_col201,
            b3_limb_9_col202,
            b3_limb_10_col203,
            c0_id_col204,
            c0_limb_0_col205,
            c0_limb_1_col206,
            c0_limb_2_col207,
            c0_limb_3_col208,
            c0_limb_4_col209,
            c0_limb_5_col210,
            c0_limb_6_col211,
            c0_limb_7_col212,
            c0_limb_8_col213,
            c0_limb_9_col214,
            c0_limb_10_col215,
            c1_id_col216,
            c1_limb_0_col217,
            c1_limb_1_col218,
            c1_limb_2_col219,
            c1_limb_3_col220,
            c1_limb_4_col221,
            c1_limb_5_col222,
            c1_limb_6_col223,
            c1_limb_7_col224,
            c1_limb_8_col225,
            c1_limb_9_col226,
            c1_limb_10_col227,
            c2_id_col228,
            c2_limb_0_col229,
            c2_limb_1_col230,
            c2_limb_2_col231,
            c2_limb_3_col232,
            c2_limb_4_col233,
            c2_limb_5_col234,
            c2_limb_6_col235,
            c2_limb_7_col236,
            c2_limb_8_col237,
            c2_limb_9_col238,
            c2_limb_10_col239,
            c3_id_col240,
            c3_limb_0_col241,
            c3_limb_1_col242,
            c3_limb_2_col243,
            c3_limb_3_col244,
            c3_limb_4_col245,
            c3_limb_5_col246,
            c3_limb_6_col247,
            c3_limb_7_col248,
            c3_limb_8_col249,
            c3_limb_9_col250,
            c3_limb_10_col251,
            sub_p_bit_col252,
            carry_0_col253,
            carry_1_col254,
            carry_2_col255,
            carry_3_col256,
            carry_4_col257,
            carry_5_col258,
            carry_6_col259,
            carry_7_col260,
            carry_8_col261,
            carry_9_col262,
            carry_10_col263,
            carry_11_col264,
            carry_12_col265,
            carry_13_col266,
        ]: [Span<QM31>; 267] =
            (*trace_mask_values
            .multi_pop_front()
            .unwrap())
            .unbox();
        let [is_instance_0_col0]: [QM31; 1] = (*is_instance_0_col0.try_into().unwrap()).unbox();
        let [p0_id_col1]: [QM31; 1] = (*p0_id_col1.try_into().unwrap()).unbox();
        let [p0_limb_0_col2]: [QM31; 1] = (*p0_limb_0_col2.try_into().unwrap()).unbox();
        let [p0_limb_1_col3]: [QM31; 1] = (*p0_limb_1_col3.try_into().unwrap()).unbox();
        let [p0_limb_2_col4]: [QM31; 1] = (*p0_limb_2_col4.try_into().unwrap()).unbox();
        let [p0_limb_3_col5]: [QM31; 1] = (*p0_limb_3_col5.try_into().unwrap()).unbox();
        let [p0_limb_4_col6]: [QM31; 1] = (*p0_limb_4_col6.try_into().unwrap()).unbox();
        let [p0_limb_5_col7]: [QM31; 1] = (*p0_limb_5_col7.try_into().unwrap()).unbox();
        let [p0_limb_6_col8]: [QM31; 1] = (*p0_limb_6_col8.try_into().unwrap()).unbox();
        let [p0_limb_7_col9]: [QM31; 1] = (*p0_limb_7_col9.try_into().unwrap()).unbox();
        let [p0_limb_8_col10]: [QM31; 1] = (*p0_limb_8_col10.try_into().unwrap()).unbox();
        let [p0_limb_9_col11]: [QM31; 1] = (*p0_limb_9_col11.try_into().unwrap()).unbox();
        let [p0_limb_10_col12]: [QM31; 1] = (*p0_limb_10_col12.try_into().unwrap()).unbox();
        let [p1_id_col13]: [QM31; 1] = (*p1_id_col13.try_into().unwrap()).unbox();
        let [p1_limb_0_col14]: [QM31; 1] = (*p1_limb_0_col14.try_into().unwrap()).unbox();
        let [p1_limb_1_col15]: [QM31; 1] = (*p1_limb_1_col15.try_into().unwrap()).unbox();
        let [p1_limb_2_col16]: [QM31; 1] = (*p1_limb_2_col16.try_into().unwrap()).unbox();
        let [p1_limb_3_col17]: [QM31; 1] = (*p1_limb_3_col17.try_into().unwrap()).unbox();
        let [p1_limb_4_col18]: [QM31; 1] = (*p1_limb_4_col18.try_into().unwrap()).unbox();
        let [p1_limb_5_col19]: [QM31; 1] = (*p1_limb_5_col19.try_into().unwrap()).unbox();
        let [p1_limb_6_col20]: [QM31; 1] = (*p1_limb_6_col20.try_into().unwrap()).unbox();
        let [p1_limb_7_col21]: [QM31; 1] = (*p1_limb_7_col21.try_into().unwrap()).unbox();
        let [p1_limb_8_col22]: [QM31; 1] = (*p1_limb_8_col22.try_into().unwrap()).unbox();
        let [p1_limb_9_col23]: [QM31; 1] = (*p1_limb_9_col23.try_into().unwrap()).unbox();
        let [p1_limb_10_col24]: [QM31; 1] = (*p1_limb_10_col24.try_into().unwrap()).unbox();
        let [p2_id_col25]: [QM31; 1] = (*p2_id_col25.try_into().unwrap()).unbox();
        let [p2_limb_0_col26]: [QM31; 1] = (*p2_limb_0_col26.try_into().unwrap()).unbox();
        let [p2_limb_1_col27]: [QM31; 1] = (*p2_limb_1_col27.try_into().unwrap()).unbox();
        let [p2_limb_2_col28]: [QM31; 1] = (*p2_limb_2_col28.try_into().unwrap()).unbox();
        let [p2_limb_3_col29]: [QM31; 1] = (*p2_limb_3_col29.try_into().unwrap()).unbox();
        let [p2_limb_4_col30]: [QM31; 1] = (*p2_limb_4_col30.try_into().unwrap()).unbox();
        let [p2_limb_5_col31]: [QM31; 1] = (*p2_limb_5_col31.try_into().unwrap()).unbox();
        let [p2_limb_6_col32]: [QM31; 1] = (*p2_limb_6_col32.try_into().unwrap()).unbox();
        let [p2_limb_7_col33]: [QM31; 1] = (*p2_limb_7_col33.try_into().unwrap()).unbox();
        let [p2_limb_8_col34]: [QM31; 1] = (*p2_limb_8_col34.try_into().unwrap()).unbox();
        let [p2_limb_9_col35]: [QM31; 1] = (*p2_limb_9_col35.try_into().unwrap()).unbox();
        let [p2_limb_10_col36]: [QM31; 1] = (*p2_limb_10_col36.try_into().unwrap()).unbox();
        let [p3_id_col37]: [QM31; 1] = (*p3_id_col37.try_into().unwrap()).unbox();
        let [p3_limb_0_col38]: [QM31; 1] = (*p3_limb_0_col38.try_into().unwrap()).unbox();
        let [p3_limb_1_col39]: [QM31; 1] = (*p3_limb_1_col39.try_into().unwrap()).unbox();
        let [p3_limb_2_col40]: [QM31; 1] = (*p3_limb_2_col40.try_into().unwrap()).unbox();
        let [p3_limb_3_col41]: [QM31; 1] = (*p3_limb_3_col41.try_into().unwrap()).unbox();
        let [p3_limb_4_col42]: [QM31; 1] = (*p3_limb_4_col42.try_into().unwrap()).unbox();
        let [p3_limb_5_col43]: [QM31; 1] = (*p3_limb_5_col43.try_into().unwrap()).unbox();
        let [p3_limb_6_col44]: [QM31; 1] = (*p3_limb_6_col44.try_into().unwrap()).unbox();
        let [p3_limb_7_col45]: [QM31; 1] = (*p3_limb_7_col45.try_into().unwrap()).unbox();
        let [p3_limb_8_col46]: [QM31; 1] = (*p3_limb_8_col46.try_into().unwrap()).unbox();
        let [p3_limb_9_col47]: [QM31; 1] = (*p3_limb_9_col47.try_into().unwrap()).unbox();
        let [p3_limb_10_col48]: [QM31; 1] = (*p3_limb_10_col48.try_into().unwrap()).unbox();
        let [values_ptr_id_col49]: [QM31; 1] = (*values_ptr_id_col49.try_into().unwrap()).unbox();
        let [values_ptr_limb_0_col50]: [QM31; 1] = (*values_ptr_limb_0_col50.try_into().unwrap())
            .unbox();
        let [values_ptr_limb_1_col51]: [QM31; 1] = (*values_ptr_limb_1_col51.try_into().unwrap())
            .unbox();
        let [values_ptr_limb_2_col52]: [QM31; 1] = (*values_ptr_limb_2_col52.try_into().unwrap())
            .unbox();
        let [values_ptr_limb_3_col53]: [QM31; 1] = (*values_ptr_limb_3_col53.try_into().unwrap())
            .unbox();
        let [partial_limb_msb_col54]: [QM31; 1] = (*partial_limb_msb_col54.try_into().unwrap())
            .unbox();
        let [offsets_ptr_id_col55]: [QM31; 1] = (*offsets_ptr_id_col55.try_into().unwrap()).unbox();
        let [offsets_ptr_limb_0_col56]: [QM31; 1] = (*offsets_ptr_limb_0_col56.try_into().unwrap())
            .unbox();
        let [offsets_ptr_limb_1_col57]: [QM31; 1] = (*offsets_ptr_limb_1_col57.try_into().unwrap())
            .unbox();
        let [offsets_ptr_limb_2_col58]: [QM31; 1] = (*offsets_ptr_limb_2_col58.try_into().unwrap())
            .unbox();
        let [offsets_ptr_limb_3_col59]: [QM31; 1] = (*offsets_ptr_limb_3_col59.try_into().unwrap())
            .unbox();
        let [partial_limb_msb_col60]: [QM31; 1] = (*partial_limb_msb_col60.try_into().unwrap())
            .unbox();
        let [offsets_ptr_prev_id_col61]: [QM31; 1] = (*offsets_ptr_prev_id_col61
            .try_into()
            .unwrap())
            .unbox();
        let [offsets_ptr_prev_limb_0_col62]: [QM31; 1] = (*offsets_ptr_prev_limb_0_col62
            .try_into()
            .unwrap())
            .unbox();
        let [offsets_ptr_prev_limb_1_col63]: [QM31; 1] = (*offsets_ptr_prev_limb_1_col63
            .try_into()
            .unwrap())
            .unbox();
        let [offsets_ptr_prev_limb_2_col64]: [QM31; 1] = (*offsets_ptr_prev_limb_2_col64
            .try_into()
            .unwrap())
            .unbox();
        let [offsets_ptr_prev_limb_3_col65]: [QM31; 1] = (*offsets_ptr_prev_limb_3_col65
            .try_into()
            .unwrap())
            .unbox();
        let [partial_limb_msb_col66]: [QM31; 1] = (*partial_limb_msb_col66.try_into().unwrap())
            .unbox();
        let [n_id_col67]: [QM31; 1] = (*n_id_col67.try_into().unwrap()).unbox();
        let [n_limb_0_col68]: [QM31; 1] = (*n_limb_0_col68.try_into().unwrap()).unbox();
        let [n_limb_1_col69]: [QM31; 1] = (*n_limb_1_col69.try_into().unwrap()).unbox();
        let [n_limb_2_col70]: [QM31; 1] = (*n_limb_2_col70.try_into().unwrap()).unbox();
        let [n_limb_3_col71]: [QM31; 1] = (*n_limb_3_col71.try_into().unwrap()).unbox();
        let [partial_limb_msb_col72]: [QM31; 1] = (*partial_limb_msb_col72.try_into().unwrap())
            .unbox();
        let [n_prev_id_col73]: [QM31; 1] = (*n_prev_id_col73.try_into().unwrap()).unbox();
        let [n_prev_limb_0_col74]: [QM31; 1] = (*n_prev_limb_0_col74.try_into().unwrap()).unbox();
        let [n_prev_limb_1_col75]: [QM31; 1] = (*n_prev_limb_1_col75.try_into().unwrap()).unbox();
        let [n_prev_limb_2_col76]: [QM31; 1] = (*n_prev_limb_2_col76.try_into().unwrap()).unbox();
        let [n_prev_limb_3_col77]: [QM31; 1] = (*n_prev_limb_3_col77.try_into().unwrap()).unbox();
        let [partial_limb_msb_col78]: [QM31; 1] = (*partial_limb_msb_col78.try_into().unwrap())
            .unbox();
        let [values_ptr_prev_id_col79]: [QM31; 1] = (*values_ptr_prev_id_col79.try_into().unwrap())
            .unbox();
        let [p_prev0_id_col80]: [QM31; 1] = (*p_prev0_id_col80.try_into().unwrap()).unbox();
        let [p_prev1_id_col81]: [QM31; 1] = (*p_prev1_id_col81.try_into().unwrap()).unbox();
        let [p_prev2_id_col82]: [QM31; 1] = (*p_prev2_id_col82.try_into().unwrap()).unbox();
        let [p_prev3_id_col83]: [QM31; 1] = (*p_prev3_id_col83.try_into().unwrap()).unbox();
        let [offsets_a_id_col84]: [QM31; 1] = (*offsets_a_id_col84.try_into().unwrap()).unbox();
        let [msb_col85]: [QM31; 1] = (*msb_col85.try_into().unwrap()).unbox();
        let [mid_limbs_set_col86]: [QM31; 1] = (*mid_limbs_set_col86.try_into().unwrap()).unbox();
        let [offsets_a_limb_0_col87]: [QM31; 1] = (*offsets_a_limb_0_col87.try_into().unwrap())
            .unbox();
        let [offsets_a_limb_1_col88]: [QM31; 1] = (*offsets_a_limb_1_col88.try_into().unwrap())
            .unbox();
        let [offsets_a_limb_2_col89]: [QM31; 1] = (*offsets_a_limb_2_col89.try_into().unwrap())
            .unbox();
        let [remainder_bits_col90]: [QM31; 1] = (*remainder_bits_col90.try_into().unwrap()).unbox();
        let [partial_limb_msb_col91]: [QM31; 1] = (*partial_limb_msb_col91.try_into().unwrap())
            .unbox();
        let [offsets_b_id_col92]: [QM31; 1] = (*offsets_b_id_col92.try_into().unwrap()).unbox();
        let [msb_col93]: [QM31; 1] = (*msb_col93.try_into().unwrap()).unbox();
        let [mid_limbs_set_col94]: [QM31; 1] = (*mid_limbs_set_col94.try_into().unwrap()).unbox();
        let [offsets_b_limb_0_col95]: [QM31; 1] = (*offsets_b_limb_0_col95.try_into().unwrap())
            .unbox();
        let [offsets_b_limb_1_col96]: [QM31; 1] = (*offsets_b_limb_1_col96.try_into().unwrap())
            .unbox();
        let [offsets_b_limb_2_col97]: [QM31; 1] = (*offsets_b_limb_2_col97.try_into().unwrap())
            .unbox();
        let [remainder_bits_col98]: [QM31; 1] = (*remainder_bits_col98.try_into().unwrap()).unbox();
        let [partial_limb_msb_col99]: [QM31; 1] = (*partial_limb_msb_col99.try_into().unwrap())
            .unbox();
        let [offsets_c_id_col100]: [QM31; 1] = (*offsets_c_id_col100.try_into().unwrap()).unbox();
        let [msb_col101]: [QM31; 1] = (*msb_col101.try_into().unwrap()).unbox();
        let [mid_limbs_set_col102]: [QM31; 1] = (*mid_limbs_set_col102.try_into().unwrap()).unbox();
        let [offsets_c_limb_0_col103]: [QM31; 1] = (*offsets_c_limb_0_col103.try_into().unwrap())
            .unbox();
        let [offsets_c_limb_1_col104]: [QM31; 1] = (*offsets_c_limb_1_col104.try_into().unwrap())
            .unbox();
        let [offsets_c_limb_2_col105]: [QM31; 1] = (*offsets_c_limb_2_col105.try_into().unwrap())
            .unbox();
        let [remainder_bits_col106]: [QM31; 1] = (*remainder_bits_col106.try_into().unwrap())
            .unbox();
        let [partial_limb_msb_col107]: [QM31; 1] = (*partial_limb_msb_col107.try_into().unwrap())
            .unbox();
        let [a0_id_col108]: [QM31; 1] = (*a0_id_col108.try_into().unwrap()).unbox();
        let [a0_limb_0_col109]: [QM31; 1] = (*a0_limb_0_col109.try_into().unwrap()).unbox();
        let [a0_limb_1_col110]: [QM31; 1] = (*a0_limb_1_col110.try_into().unwrap()).unbox();
        let [a0_limb_2_col111]: [QM31; 1] = (*a0_limb_2_col111.try_into().unwrap()).unbox();
        let [a0_limb_3_col112]: [QM31; 1] = (*a0_limb_3_col112.try_into().unwrap()).unbox();
        let [a0_limb_4_col113]: [QM31; 1] = (*a0_limb_4_col113.try_into().unwrap()).unbox();
        let [a0_limb_5_col114]: [QM31; 1] = (*a0_limb_5_col114.try_into().unwrap()).unbox();
        let [a0_limb_6_col115]: [QM31; 1] = (*a0_limb_6_col115.try_into().unwrap()).unbox();
        let [a0_limb_7_col116]: [QM31; 1] = (*a0_limb_7_col116.try_into().unwrap()).unbox();
        let [a0_limb_8_col117]: [QM31; 1] = (*a0_limb_8_col117.try_into().unwrap()).unbox();
        let [a0_limb_9_col118]: [QM31; 1] = (*a0_limb_9_col118.try_into().unwrap()).unbox();
        let [a0_limb_10_col119]: [QM31; 1] = (*a0_limb_10_col119.try_into().unwrap()).unbox();
        let [a1_id_col120]: [QM31; 1] = (*a1_id_col120.try_into().unwrap()).unbox();
        let [a1_limb_0_col121]: [QM31; 1] = (*a1_limb_0_col121.try_into().unwrap()).unbox();
        let [a1_limb_1_col122]: [QM31; 1] = (*a1_limb_1_col122.try_into().unwrap()).unbox();
        let [a1_limb_2_col123]: [QM31; 1] = (*a1_limb_2_col123.try_into().unwrap()).unbox();
        let [a1_limb_3_col124]: [QM31; 1] = (*a1_limb_3_col124.try_into().unwrap()).unbox();
        let [a1_limb_4_col125]: [QM31; 1] = (*a1_limb_4_col125.try_into().unwrap()).unbox();
        let [a1_limb_5_col126]: [QM31; 1] = (*a1_limb_5_col126.try_into().unwrap()).unbox();
        let [a1_limb_6_col127]: [QM31; 1] = (*a1_limb_6_col127.try_into().unwrap()).unbox();
        let [a1_limb_7_col128]: [QM31; 1] = (*a1_limb_7_col128.try_into().unwrap()).unbox();
        let [a1_limb_8_col129]: [QM31; 1] = (*a1_limb_8_col129.try_into().unwrap()).unbox();
        let [a1_limb_9_col130]: [QM31; 1] = (*a1_limb_9_col130.try_into().unwrap()).unbox();
        let [a1_limb_10_col131]: [QM31; 1] = (*a1_limb_10_col131.try_into().unwrap()).unbox();
        let [a2_id_col132]: [QM31; 1] = (*a2_id_col132.try_into().unwrap()).unbox();
        let [a2_limb_0_col133]: [QM31; 1] = (*a2_limb_0_col133.try_into().unwrap()).unbox();
        let [a2_limb_1_col134]: [QM31; 1] = (*a2_limb_1_col134.try_into().unwrap()).unbox();
        let [a2_limb_2_col135]: [QM31; 1] = (*a2_limb_2_col135.try_into().unwrap()).unbox();
        let [a2_limb_3_col136]: [QM31; 1] = (*a2_limb_3_col136.try_into().unwrap()).unbox();
        let [a2_limb_4_col137]: [QM31; 1] = (*a2_limb_4_col137.try_into().unwrap()).unbox();
        let [a2_limb_5_col138]: [QM31; 1] = (*a2_limb_5_col138.try_into().unwrap()).unbox();
        let [a2_limb_6_col139]: [QM31; 1] = (*a2_limb_6_col139.try_into().unwrap()).unbox();
        let [a2_limb_7_col140]: [QM31; 1] = (*a2_limb_7_col140.try_into().unwrap()).unbox();
        let [a2_limb_8_col141]: [QM31; 1] = (*a2_limb_8_col141.try_into().unwrap()).unbox();
        let [a2_limb_9_col142]: [QM31; 1] = (*a2_limb_9_col142.try_into().unwrap()).unbox();
        let [a2_limb_10_col143]: [QM31; 1] = (*a2_limb_10_col143.try_into().unwrap()).unbox();
        let [a3_id_col144]: [QM31; 1] = (*a3_id_col144.try_into().unwrap()).unbox();
        let [a3_limb_0_col145]: [QM31; 1] = (*a3_limb_0_col145.try_into().unwrap()).unbox();
        let [a3_limb_1_col146]: [QM31; 1] = (*a3_limb_1_col146.try_into().unwrap()).unbox();
        let [a3_limb_2_col147]: [QM31; 1] = (*a3_limb_2_col147.try_into().unwrap()).unbox();
        let [a3_limb_3_col148]: [QM31; 1] = (*a3_limb_3_col148.try_into().unwrap()).unbox();
        let [a3_limb_4_col149]: [QM31; 1] = (*a3_limb_4_col149.try_into().unwrap()).unbox();
        let [a3_limb_5_col150]: [QM31; 1] = (*a3_limb_5_col150.try_into().unwrap()).unbox();
        let [a3_limb_6_col151]: [QM31; 1] = (*a3_limb_6_col151.try_into().unwrap()).unbox();
        let [a3_limb_7_col152]: [QM31; 1] = (*a3_limb_7_col152.try_into().unwrap()).unbox();
        let [a3_limb_8_col153]: [QM31; 1] = (*a3_limb_8_col153.try_into().unwrap()).unbox();
        let [a3_limb_9_col154]: [QM31; 1] = (*a3_limb_9_col154.try_into().unwrap()).unbox();
        let [a3_limb_10_col155]: [QM31; 1] = (*a3_limb_10_col155.try_into().unwrap()).unbox();
        let [b0_id_col156]: [QM31; 1] = (*b0_id_col156.try_into().unwrap()).unbox();
        let [b0_limb_0_col157]: [QM31; 1] = (*b0_limb_0_col157.try_into().unwrap()).unbox();
        let [b0_limb_1_col158]: [QM31; 1] = (*b0_limb_1_col158.try_into().unwrap()).unbox();
        let [b0_limb_2_col159]: [QM31; 1] = (*b0_limb_2_col159.try_into().unwrap()).unbox();
        let [b0_limb_3_col160]: [QM31; 1] = (*b0_limb_3_col160.try_into().unwrap()).unbox();
        let [b0_limb_4_col161]: [QM31; 1] = (*b0_limb_4_col161.try_into().unwrap()).unbox();
        let [b0_limb_5_col162]: [QM31; 1] = (*b0_limb_5_col162.try_into().unwrap()).unbox();
        let [b0_limb_6_col163]: [QM31; 1] = (*b0_limb_6_col163.try_into().unwrap()).unbox();
        let [b0_limb_7_col164]: [QM31; 1] = (*b0_limb_7_col164.try_into().unwrap()).unbox();
        let [b0_limb_8_col165]: [QM31; 1] = (*b0_limb_8_col165.try_into().unwrap()).unbox();
        let [b0_limb_9_col166]: [QM31; 1] = (*b0_limb_9_col166.try_into().unwrap()).unbox();
        let [b0_limb_10_col167]: [QM31; 1] = (*b0_limb_10_col167.try_into().unwrap()).unbox();
        let [b1_id_col168]: [QM31; 1] = (*b1_id_col168.try_into().unwrap()).unbox();
        let [b1_limb_0_col169]: [QM31; 1] = (*b1_limb_0_col169.try_into().unwrap()).unbox();
        let [b1_limb_1_col170]: [QM31; 1] = (*b1_limb_1_col170.try_into().unwrap()).unbox();
        let [b1_limb_2_col171]: [QM31; 1] = (*b1_limb_2_col171.try_into().unwrap()).unbox();
        let [b1_limb_3_col172]: [QM31; 1] = (*b1_limb_3_col172.try_into().unwrap()).unbox();
        let [b1_limb_4_col173]: [QM31; 1] = (*b1_limb_4_col173.try_into().unwrap()).unbox();
        let [b1_limb_5_col174]: [QM31; 1] = (*b1_limb_5_col174.try_into().unwrap()).unbox();
        let [b1_limb_6_col175]: [QM31; 1] = (*b1_limb_6_col175.try_into().unwrap()).unbox();
        let [b1_limb_7_col176]: [QM31; 1] = (*b1_limb_7_col176.try_into().unwrap()).unbox();
        let [b1_limb_8_col177]: [QM31; 1] = (*b1_limb_8_col177.try_into().unwrap()).unbox();
        let [b1_limb_9_col178]: [QM31; 1] = (*b1_limb_9_col178.try_into().unwrap()).unbox();
        let [b1_limb_10_col179]: [QM31; 1] = (*b1_limb_10_col179.try_into().unwrap()).unbox();
        let [b2_id_col180]: [QM31; 1] = (*b2_id_col180.try_into().unwrap()).unbox();
        let [b2_limb_0_col181]: [QM31; 1] = (*b2_limb_0_col181.try_into().unwrap()).unbox();
        let [b2_limb_1_col182]: [QM31; 1] = (*b2_limb_1_col182.try_into().unwrap()).unbox();
        let [b2_limb_2_col183]: [QM31; 1] = (*b2_limb_2_col183.try_into().unwrap()).unbox();
        let [b2_limb_3_col184]: [QM31; 1] = (*b2_limb_3_col184.try_into().unwrap()).unbox();
        let [b2_limb_4_col185]: [QM31; 1] = (*b2_limb_4_col185.try_into().unwrap()).unbox();
        let [b2_limb_5_col186]: [QM31; 1] = (*b2_limb_5_col186.try_into().unwrap()).unbox();
        let [b2_limb_6_col187]: [QM31; 1] = (*b2_limb_6_col187.try_into().unwrap()).unbox();
        let [b2_limb_7_col188]: [QM31; 1] = (*b2_limb_7_col188.try_into().unwrap()).unbox();
        let [b2_limb_8_col189]: [QM31; 1] = (*b2_limb_8_col189.try_into().unwrap()).unbox();
        let [b2_limb_9_col190]: [QM31; 1] = (*b2_limb_9_col190.try_into().unwrap()).unbox();
        let [b2_limb_10_col191]: [QM31; 1] = (*b2_limb_10_col191.try_into().unwrap()).unbox();
        let [b3_id_col192]: [QM31; 1] = (*b3_id_col192.try_into().unwrap()).unbox();
        let [b3_limb_0_col193]: [QM31; 1] = (*b3_limb_0_col193.try_into().unwrap()).unbox();
        let [b3_limb_1_col194]: [QM31; 1] = (*b3_limb_1_col194.try_into().unwrap()).unbox();
        let [b3_limb_2_col195]: [QM31; 1] = (*b3_limb_2_col195.try_into().unwrap()).unbox();
        let [b3_limb_3_col196]: [QM31; 1] = (*b3_limb_3_col196.try_into().unwrap()).unbox();
        let [b3_limb_4_col197]: [QM31; 1] = (*b3_limb_4_col197.try_into().unwrap()).unbox();
        let [b3_limb_5_col198]: [QM31; 1] = (*b3_limb_5_col198.try_into().unwrap()).unbox();
        let [b3_limb_6_col199]: [QM31; 1] = (*b3_limb_6_col199.try_into().unwrap()).unbox();
        let [b3_limb_7_col200]: [QM31; 1] = (*b3_limb_7_col200.try_into().unwrap()).unbox();
        let [b3_limb_8_col201]: [QM31; 1] = (*b3_limb_8_col201.try_into().unwrap()).unbox();
        let [b3_limb_9_col202]: [QM31; 1] = (*b3_limb_9_col202.try_into().unwrap()).unbox();
        let [b3_limb_10_col203]: [QM31; 1] = (*b3_limb_10_col203.try_into().unwrap()).unbox();
        let [c0_id_col204]: [QM31; 1] = (*c0_id_col204.try_into().unwrap()).unbox();
        let [c0_limb_0_col205]: [QM31; 1] = (*c0_limb_0_col205.try_into().unwrap()).unbox();
        let [c0_limb_1_col206]: [QM31; 1] = (*c0_limb_1_col206.try_into().unwrap()).unbox();
        let [c0_limb_2_col207]: [QM31; 1] = (*c0_limb_2_col207.try_into().unwrap()).unbox();
        let [c0_limb_3_col208]: [QM31; 1] = (*c0_limb_3_col208.try_into().unwrap()).unbox();
        let [c0_limb_4_col209]: [QM31; 1] = (*c0_limb_4_col209.try_into().unwrap()).unbox();
        let [c0_limb_5_col210]: [QM31; 1] = (*c0_limb_5_col210.try_into().unwrap()).unbox();
        let [c0_limb_6_col211]: [QM31; 1] = (*c0_limb_6_col211.try_into().unwrap()).unbox();
        let [c0_limb_7_col212]: [QM31; 1] = (*c0_limb_7_col212.try_into().unwrap()).unbox();
        let [c0_limb_8_col213]: [QM31; 1] = (*c0_limb_8_col213.try_into().unwrap()).unbox();
        let [c0_limb_9_col214]: [QM31; 1] = (*c0_limb_9_col214.try_into().unwrap()).unbox();
        let [c0_limb_10_col215]: [QM31; 1] = (*c0_limb_10_col215.try_into().unwrap()).unbox();
        let [c1_id_col216]: [QM31; 1] = (*c1_id_col216.try_into().unwrap()).unbox();
        let [c1_limb_0_col217]: [QM31; 1] = (*c1_limb_0_col217.try_into().unwrap()).unbox();
        let [c1_limb_1_col218]: [QM31; 1] = (*c1_limb_1_col218.try_into().unwrap()).unbox();
        let [c1_limb_2_col219]: [QM31; 1] = (*c1_limb_2_col219.try_into().unwrap()).unbox();
        let [c1_limb_3_col220]: [QM31; 1] = (*c1_limb_3_col220.try_into().unwrap()).unbox();
        let [c1_limb_4_col221]: [QM31; 1] = (*c1_limb_4_col221.try_into().unwrap()).unbox();
        let [c1_limb_5_col222]: [QM31; 1] = (*c1_limb_5_col222.try_into().unwrap()).unbox();
        let [c1_limb_6_col223]: [QM31; 1] = (*c1_limb_6_col223.try_into().unwrap()).unbox();
        let [c1_limb_7_col224]: [QM31; 1] = (*c1_limb_7_col224.try_into().unwrap()).unbox();
        let [c1_limb_8_col225]: [QM31; 1] = (*c1_limb_8_col225.try_into().unwrap()).unbox();
        let [c1_limb_9_col226]: [QM31; 1] = (*c1_limb_9_col226.try_into().unwrap()).unbox();
        let [c1_limb_10_col227]: [QM31; 1] = (*c1_limb_10_col227.try_into().unwrap()).unbox();
        let [c2_id_col228]: [QM31; 1] = (*c2_id_col228.try_into().unwrap()).unbox();
        let [c2_limb_0_col229]: [QM31; 1] = (*c2_limb_0_col229.try_into().unwrap()).unbox();
        let [c2_limb_1_col230]: [QM31; 1] = (*c2_limb_1_col230.try_into().unwrap()).unbox();
        let [c2_limb_2_col231]: [QM31; 1] = (*c2_limb_2_col231.try_into().unwrap()).unbox();
        let [c2_limb_3_col232]: [QM31; 1] = (*c2_limb_3_col232.try_into().unwrap()).unbox();
        let [c2_limb_4_col233]: [QM31; 1] = (*c2_limb_4_col233.try_into().unwrap()).unbox();
        let [c2_limb_5_col234]: [QM31; 1] = (*c2_limb_5_col234.try_into().unwrap()).unbox();
        let [c2_limb_6_col235]: [QM31; 1] = (*c2_limb_6_col235.try_into().unwrap()).unbox();
        let [c2_limb_7_col236]: [QM31; 1] = (*c2_limb_7_col236.try_into().unwrap()).unbox();
        let [c2_limb_8_col237]: [QM31; 1] = (*c2_limb_8_col237.try_into().unwrap()).unbox();
        let [c2_limb_9_col238]: [QM31; 1] = (*c2_limb_9_col238.try_into().unwrap()).unbox();
        let [c2_limb_10_col239]: [QM31; 1] = (*c2_limb_10_col239.try_into().unwrap()).unbox();
        let [c3_id_col240]: [QM31; 1] = (*c3_id_col240.try_into().unwrap()).unbox();
        let [c3_limb_0_col241]: [QM31; 1] = (*c3_limb_0_col241.try_into().unwrap()).unbox();
        let [c3_limb_1_col242]: [QM31; 1] = (*c3_limb_1_col242.try_into().unwrap()).unbox();
        let [c3_limb_2_col243]: [QM31; 1] = (*c3_limb_2_col243.try_into().unwrap()).unbox();
        let [c3_limb_3_col244]: [QM31; 1] = (*c3_limb_3_col244.try_into().unwrap()).unbox();
        let [c3_limb_4_col245]: [QM31; 1] = (*c3_limb_4_col245.try_into().unwrap()).unbox();
        let [c3_limb_5_col246]: [QM31; 1] = (*c3_limb_5_col246.try_into().unwrap()).unbox();
        let [c3_limb_6_col247]: [QM31; 1] = (*c3_limb_6_col247.try_into().unwrap()).unbox();
        let [c3_limb_7_col248]: [QM31; 1] = (*c3_limb_7_col248.try_into().unwrap()).unbox();
        let [c3_limb_8_col249]: [QM31; 1] = (*c3_limb_8_col249.try_into().unwrap()).unbox();
        let [c3_limb_9_col250]: [QM31; 1] = (*c3_limb_9_col250.try_into().unwrap()).unbox();
        let [c3_limb_10_col251]: [QM31; 1] = (*c3_limb_10_col251.try_into().unwrap()).unbox();
        let [sub_p_bit_col252]: [QM31; 1] = (*sub_p_bit_col252.try_into().unwrap()).unbox();
        let [carry_0_col253]: [QM31; 1] = (*carry_0_col253.try_into().unwrap()).unbox();
        let [carry_1_col254]: [QM31; 1] = (*carry_1_col254.try_into().unwrap()).unbox();
        let [carry_2_col255]: [QM31; 1] = (*carry_2_col255.try_into().unwrap()).unbox();
        let [carry_3_col256]: [QM31; 1] = (*carry_3_col256.try_into().unwrap()).unbox();
        let [carry_4_col257]: [QM31; 1] = (*carry_4_col257.try_into().unwrap()).unbox();
        let [carry_5_col258]: [QM31; 1] = (*carry_5_col258.try_into().unwrap()).unbox();
        let [carry_6_col259]: [QM31; 1] = (*carry_6_col259.try_into().unwrap()).unbox();
        let [carry_7_col260]: [QM31; 1] = (*carry_7_col260.try_into().unwrap()).unbox();
        let [carry_8_col261]: [QM31; 1] = (*carry_8_col261.try_into().unwrap()).unbox();
        let [carry_9_col262]: [QM31; 1] = (*carry_9_col262.try_into().unwrap()).unbox();
        let [carry_10_col263]: [QM31; 1] = (*carry_10_col263.try_into().unwrap()).unbox();
        let [carry_11_col264]: [QM31; 1] = (*carry_11_col264.try_into().unwrap()).unbox();
        let [carry_12_col265]: [QM31; 1] = (*carry_12_col265.try_into().unwrap()).unbox();
        let [carry_13_col266]: [QM31; 1] = (*carry_13_col266.try_into().unwrap()).unbox();

        core::internal::revoke_ap_tracking();

        mod_utils_evaluate(
            [add_mod_builtin_segment_start, seq],
            is_instance_0_col0,
            p0_id_col1,
            p0_limb_0_col2,
            p0_limb_1_col3,
            p0_limb_2_col4,
            p0_limb_3_col5,
            p0_limb_4_col6,
            p0_limb_5_col7,
            p0_limb_6_col8,
            p0_limb_7_col9,
            p0_limb_8_col10,
            p0_limb_9_col11,
            p0_limb_10_col12,
            p1_id_col13,
            p1_limb_0_col14,
            p1_limb_1_col15,
            p1_limb_2_col16,
            p1_limb_3_col17,
            p1_limb_4_col18,
            p1_limb_5_col19,
            p1_limb_6_col20,
            p1_limb_7_col21,
            p1_limb_8_col22,
            p1_limb_9_col23,
            p1_limb_10_col24,
            p2_id_col25,
            p2_limb_0_col26,
            p2_limb_1_col27,
            p2_limb_2_col28,
            p2_limb_3_col29,
            p2_limb_4_col30,
            p2_limb_5_col31,
            p2_limb_6_col32,
            p2_limb_7_col33,
            p2_limb_8_col34,
            p2_limb_9_col35,
            p2_limb_10_col36,
            p3_id_col37,
            p3_limb_0_col38,
            p3_limb_1_col39,
            p3_limb_2_col40,
            p3_limb_3_col41,
            p3_limb_4_col42,
            p3_limb_5_col43,
            p3_limb_6_col44,
            p3_limb_7_col45,
            p3_limb_8_col46,
            p3_limb_9_col47,
            p3_limb_10_col48,
            values_ptr_id_col49,
            values_ptr_limb_0_col50,
            values_ptr_limb_1_col51,
            values_ptr_limb_2_col52,
            values_ptr_limb_3_col53,
            partial_limb_msb_col54,
            offsets_ptr_id_col55,
            offsets_ptr_limb_0_col56,
            offsets_ptr_limb_1_col57,
            offsets_ptr_limb_2_col58,
            offsets_ptr_limb_3_col59,
            partial_limb_msb_col60,
            offsets_ptr_prev_id_col61,
            offsets_ptr_prev_limb_0_col62,
            offsets_ptr_prev_limb_1_col63,
            offsets_ptr_prev_limb_2_col64,
            offsets_ptr_prev_limb_3_col65,
            partial_limb_msb_col66,
            n_id_col67,
            n_limb_0_col68,
            n_limb_1_col69,
            n_limb_2_col70,
            n_limb_3_col71,
            partial_limb_msb_col72,
            n_prev_id_col73,
            n_prev_limb_0_col74,
            n_prev_limb_1_col75,
            n_prev_limb_2_col76,
            n_prev_limb_3_col77,
            partial_limb_msb_col78,
            values_ptr_prev_id_col79,
            p_prev0_id_col80,
            p_prev1_id_col81,
            p_prev2_id_col82,
            p_prev3_id_col83,
            offsets_a_id_col84,
            msb_col85,
            mid_limbs_set_col86,
            offsets_a_limb_0_col87,
            offsets_a_limb_1_col88,
            offsets_a_limb_2_col89,
            remainder_bits_col90,
            partial_limb_msb_col91,
            offsets_b_id_col92,
            msb_col93,
            mid_limbs_set_col94,
            offsets_b_limb_0_col95,
            offsets_b_limb_1_col96,
            offsets_b_limb_2_col97,
            remainder_bits_col98,
            partial_limb_msb_col99,
            offsets_c_id_col100,
            msb_col101,
            mid_limbs_set_col102,
            offsets_c_limb_0_col103,
            offsets_c_limb_1_col104,
            offsets_c_limb_2_col105,
            remainder_bits_col106,
            partial_limb_msb_col107,
            a0_id_col108,
            a0_limb_0_col109,
            a0_limb_1_col110,
            a0_limb_2_col111,
            a0_limb_3_col112,
            a0_limb_4_col113,
            a0_limb_5_col114,
            a0_limb_6_col115,
            a0_limb_7_col116,
            a0_limb_8_col117,
            a0_limb_9_col118,
            a0_limb_10_col119,
            a1_id_col120,
            a1_limb_0_col121,
            a1_limb_1_col122,
            a1_limb_2_col123,
            a1_limb_3_col124,
            a1_limb_4_col125,
            a1_limb_5_col126,
            a1_limb_6_col127,
            a1_limb_7_col128,
            a1_limb_8_col129,
            a1_limb_9_col130,
            a1_limb_10_col131,
            a2_id_col132,
            a2_limb_0_col133,
            a2_limb_1_col134,
            a2_limb_2_col135,
            a2_limb_3_col136,
            a2_limb_4_col137,
            a2_limb_5_col138,
            a2_limb_6_col139,
            a2_limb_7_col140,
            a2_limb_8_col141,
            a2_limb_9_col142,
            a2_limb_10_col143,
            a3_id_col144,
            a3_limb_0_col145,
            a3_limb_1_col146,
            a3_limb_2_col147,
            a3_limb_3_col148,
            a3_limb_4_col149,
            a3_limb_5_col150,
            a3_limb_6_col151,
            a3_limb_7_col152,
            a3_limb_8_col153,
            a3_limb_9_col154,
            a3_limb_10_col155,
            b0_id_col156,
            b0_limb_0_col157,
            b0_limb_1_col158,
            b0_limb_2_col159,
            b0_limb_3_col160,
            b0_limb_4_col161,
            b0_limb_5_col162,
            b0_limb_6_col163,
            b0_limb_7_col164,
            b0_limb_8_col165,
            b0_limb_9_col166,
            b0_limb_10_col167,
            b1_id_col168,
            b1_limb_0_col169,
            b1_limb_1_col170,
            b1_limb_2_col171,
            b1_limb_3_col172,
            b1_limb_4_col173,
            b1_limb_5_col174,
            b1_limb_6_col175,
            b1_limb_7_col176,
            b1_limb_8_col177,
            b1_limb_9_col178,
            b1_limb_10_col179,
            b2_id_col180,
            b2_limb_0_col181,
            b2_limb_1_col182,
            b2_limb_2_col183,
            b2_limb_3_col184,
            b2_limb_4_col185,
            b2_limb_5_col186,
            b2_limb_6_col187,
            b2_limb_7_col188,
            b2_limb_8_col189,
            b2_limb_9_col190,
            b2_limb_10_col191,
            b3_id_col192,
            b3_limb_0_col193,
            b3_limb_1_col194,
            b3_limb_2_col195,
            b3_limb_3_col196,
            b3_limb_4_col197,
            b3_limb_5_col198,
            b3_limb_6_col199,
            b3_limb_7_col200,
            b3_limb_8_col201,
            b3_limb_9_col202,
            b3_limb_10_col203,
            c0_id_col204,
            c0_limb_0_col205,
            c0_limb_1_col206,
            c0_limb_2_col207,
            c0_limb_3_col208,
            c0_limb_4_col209,
            c0_limb_5_col210,
            c0_limb_6_col211,
            c0_limb_7_col212,
            c0_limb_8_col213,
            c0_limb_9_col214,
            c0_limb_10_col215,
            c1_id_col216,
            c1_limb_0_col217,
            c1_limb_1_col218,
            c1_limb_2_col219,
            c1_limb_3_col220,
            c1_limb_4_col221,
            c1_limb_5_col222,
            c1_limb_6_col223,
            c1_limb_7_col224,
            c1_limb_8_col225,
            c1_limb_9_col226,
            c1_limb_10_col227,
            c2_id_col228,
            c2_limb_0_col229,
            c2_limb_1_col230,
            c2_limb_2_col231,
            c2_limb_3_col232,
            c2_limb_4_col233,
            c2_limb_5_col234,
            c2_limb_6_col235,
            c2_limb_7_col236,
            c2_limb_8_col237,
            c2_limb_9_col238,
            c2_limb_10_col239,
            c3_id_col240,
            c3_limb_0_col241,
            c3_limb_1_col242,
            c3_limb_2_col243,
            c3_limb_3_col244,
            c3_limb_4_col245,
            c3_limb_5_col246,
            c3_limb_6_col247,
            c3_limb_7_col248,
            c3_limb_8_col249,
            c3_limb_9_col250,
            c3_limb_10_col251,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref memory_address_to_id_sum_0,
            ref memory_id_to_big_sum_1,
            ref memory_address_to_id_sum_2,
            ref memory_id_to_big_sum_3,
            ref memory_address_to_id_sum_4,
            ref memory_id_to_big_sum_5,
            ref memory_address_to_id_sum_6,
            ref memory_id_to_big_sum_7,
            ref memory_address_to_id_sum_8,
            ref memory_id_to_big_sum_9,
            ref memory_address_to_id_sum_10,
            ref memory_id_to_big_sum_11,
            ref memory_address_to_id_sum_12,
            ref memory_id_to_big_sum_13,
            ref memory_address_to_id_sum_14,
            ref memory_id_to_big_sum_15,
            ref memory_address_to_id_sum_16,
            ref memory_id_to_big_sum_17,
            ref memory_address_to_id_sum_18,
            ref memory_address_to_id_sum_19,
            ref memory_address_to_id_sum_20,
            ref memory_address_to_id_sum_21,
            ref memory_address_to_id_sum_22,
            ref memory_address_to_id_sum_23,
            ref memory_id_to_big_sum_24,
            ref memory_address_to_id_sum_25,
            ref memory_id_to_big_sum_26,
            ref memory_address_to_id_sum_27,
            ref memory_id_to_big_sum_28,
            ref memory_address_to_id_sum_29,
            ref memory_id_to_big_sum_30,
            ref memory_address_to_id_sum_31,
            ref memory_id_to_big_sum_32,
            ref memory_address_to_id_sum_33,
            ref memory_id_to_big_sum_34,
            ref memory_address_to_id_sum_35,
            ref memory_id_to_big_sum_36,
            ref memory_address_to_id_sum_37,
            ref memory_id_to_big_sum_38,
            ref memory_address_to_id_sum_39,
            ref memory_id_to_big_sum_40,
            ref memory_address_to_id_sum_41,
            ref memory_id_to_big_sum_42,
            ref memory_address_to_id_sum_43,
            ref memory_id_to_big_sum_44,
            ref memory_address_to_id_sum_45,
            ref memory_id_to_big_sum_46,
            ref memory_address_to_id_sum_47,
            ref memory_id_to_big_sum_48,
            ref memory_address_to_id_sum_49,
            ref memory_id_to_big_sum_50,
            ref memory_address_to_id_sum_51,
            ref memory_id_to_big_sum_52,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        // Constraint - make sure sub_p_bit is 0 or 1.
        let constraint_quotient = (((sub_p_bit_col252 - qm31_const::<1, 0, 0, 0>())
            * sub_p_bit_col252))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - carry_0
        let constraint_quotient = ((carry_0_col253
            - ((((((a0_limb_0_col109 + b0_limb_0_col157) - c0_limb_0_col205)
                - (p0_limb_0_col2 * sub_p_bit_col252))
                + (qm31_const::<512, 0, 0, 0>()
                    * (((a0_limb_1_col110 + b0_limb_1_col158) - c0_limb_1_col206)
                        - (p0_limb_1_col3 * sub_p_bit_col252))))
                + (qm31_const::<262144, 0, 0, 0>()
                    * (((a0_limb_2_col111 + b0_limb_2_col159) - c0_limb_2_col207)
                        - (p0_limb_2_col4 * sub_p_bit_col252))))
                * qm31_const::<16, 0, 0, 0>())))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - carry is 0 or 1 or -1.
        let constraint_quotient = ((carry_0_col253
            * ((carry_0_col253 * carry_0_col253) - qm31_const::<1, 0, 0, 0>())))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - carry_1
        let constraint_quotient = ((carry_1_col254
            - ((((carry_0_col253
                + (((a0_limb_3_col112 + b0_limb_3_col160) - c0_limb_3_col208)
                    - (p0_limb_3_col5 * sub_p_bit_col252)))
                + (qm31_const::<512, 0, 0, 0>()
                    * (((a0_limb_4_col113 + b0_limb_4_col161) - c0_limb_4_col209)
                        - (p0_limb_4_col6 * sub_p_bit_col252))))
                + (qm31_const::<262144, 0, 0, 0>()
                    * (((a0_limb_5_col114 + b0_limb_5_col162) - c0_limb_5_col210)
                        - (p0_limb_5_col7 * sub_p_bit_col252))))
                * qm31_const::<16, 0, 0, 0>())))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - carry is 0 or 1 or -1.
        let constraint_quotient = ((carry_1_col254
            * ((carry_1_col254 * carry_1_col254) - qm31_const::<1, 0, 0, 0>())))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - carry_2
        let constraint_quotient = ((carry_2_col255
            - ((((carry_1_col254
                + (((a0_limb_6_col115 + b0_limb_6_col163) - c0_limb_6_col211)
                    - (p0_limb_6_col8 * sub_p_bit_col252)))
                + (qm31_const::<512, 0, 0, 0>()
                    * (((a0_limb_7_col116 + b0_limb_7_col164) - c0_limb_7_col212)
                        - (p0_limb_7_col9 * sub_p_bit_col252))))
                + (qm31_const::<262144, 0, 0, 0>()
                    * (((a0_limb_8_col117 + b0_limb_8_col165) - c0_limb_8_col213)
                        - (p0_limb_8_col10 * sub_p_bit_col252))))
                * qm31_const::<16, 0, 0, 0>())))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - carry is 0 or 1 or -1.
        let constraint_quotient = ((carry_2_col255
            * ((carry_2_col255 * carry_2_col255) - qm31_const::<1, 0, 0, 0>())))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - carry_3
        let constraint_quotient = ((carry_3_col256
            - ((((carry_2_col255
                + (((a0_limb_9_col118 + b0_limb_9_col166) - c0_limb_9_col214)
                    - (p0_limb_9_col11 * sub_p_bit_col252)))
                + (qm31_const::<512, 0, 0, 0>()
                    * (((a0_limb_10_col119 + b0_limb_10_col167) - c0_limb_10_col215)
                        - (p0_limb_10_col12 * sub_p_bit_col252))))
                + (qm31_const::<32768, 0, 0, 0>()
                    * (((a1_limb_0_col121 + b1_limb_0_col169) - c1_limb_0_col217)
                        - (p1_limb_0_col14 * sub_p_bit_col252))))
                * qm31_const::<128, 0, 0, 0>())))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - carry is 0 or 1 or -1.
        let constraint_quotient = ((carry_3_col256
            * ((carry_3_col256 * carry_3_col256) - qm31_const::<1, 0, 0, 0>())))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - carry_4
        let constraint_quotient = ((carry_4_col257
            - ((((carry_3_col256
                + (((a1_limb_1_col122 + b1_limb_1_col170) - c1_limb_1_col218)
                    - (p1_limb_1_col15 * sub_p_bit_col252)))
                + (qm31_const::<512, 0, 0, 0>()
                    * (((a1_limb_2_col123 + b1_limb_2_col171) - c1_limb_2_col219)
                        - (p1_limb_2_col16 * sub_p_bit_col252))))
                + (qm31_const::<262144, 0, 0, 0>()
                    * (((a1_limb_3_col124 + b1_limb_3_col172) - c1_limb_3_col220)
                        - (p1_limb_3_col17 * sub_p_bit_col252))))
                * qm31_const::<16, 0, 0, 0>())))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - carry is 0 or 1 or -1.
        let constraint_quotient = ((carry_4_col257
            * ((carry_4_col257 * carry_4_col257) - qm31_const::<1, 0, 0, 0>())))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - carry_5
        let constraint_quotient = ((carry_5_col258
            - ((((carry_4_col257
                + (((a1_limb_4_col125 + b1_limb_4_col173) - c1_limb_4_col221)
                    - (p1_limb_4_col18 * sub_p_bit_col252)))
                + (qm31_const::<512, 0, 0, 0>()
                    * (((a1_limb_5_col126 + b1_limb_5_col174) - c1_limb_5_col222)
                        - (p1_limb_5_col19 * sub_p_bit_col252))))
                + (qm31_const::<262144, 0, 0, 0>()
                    * (((a1_limb_6_col127 + b1_limb_6_col175) - c1_limb_6_col223)
                        - (p1_limb_6_col20 * sub_p_bit_col252))))
                * qm31_const::<16, 0, 0, 0>())))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - carry is 0 or 1 or -1.
        let constraint_quotient = ((carry_5_col258
            * ((carry_5_col258 * carry_5_col258) - qm31_const::<1, 0, 0, 0>())))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - carry_6
        let constraint_quotient = ((carry_6_col259
            - ((((carry_5_col258
                + (((a1_limb_7_col128 + b1_limb_7_col176) - c1_limb_7_col224)
                    - (p1_limb_7_col21 * sub_p_bit_col252)))
                + (qm31_const::<512, 0, 0, 0>()
                    * (((a1_limb_8_col129 + b1_limb_8_col177) - c1_limb_8_col225)
                        - (p1_limb_8_col22 * sub_p_bit_col252))))
                + (qm31_const::<262144, 0, 0, 0>()
                    * (((a1_limb_9_col130 + b1_limb_9_col178) - c1_limb_9_col226)
                        - (p1_limb_9_col23 * sub_p_bit_col252))))
                * qm31_const::<16, 0, 0, 0>())))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - carry is 0 or 1 or -1.
        let constraint_quotient = ((carry_6_col259
            * ((carry_6_col259 * carry_6_col259) - qm31_const::<1, 0, 0, 0>())))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - carry_7
        let constraint_quotient = ((carry_7_col260
            - ((((carry_6_col259
                + (((a1_limb_10_col131 + b1_limb_10_col179) - c1_limb_10_col227)
                    - (p1_limb_10_col24 * sub_p_bit_col252)))
                + (qm31_const::<64, 0, 0, 0>()
                    * (((a2_limb_0_col133 + b2_limb_0_col181) - c2_limb_0_col229)
                        - (p2_limb_0_col26 * sub_p_bit_col252))))
                + (qm31_const::<32768, 0, 0, 0>()
                    * (((a2_limb_1_col134 + b2_limb_1_col182) - c2_limb_1_col230)
                        - (p2_limb_1_col27 * sub_p_bit_col252))))
                * qm31_const::<128, 0, 0, 0>())))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - carry is 0 or 1 or -1.
        let constraint_quotient = ((carry_7_col260
            * ((carry_7_col260 * carry_7_col260) - qm31_const::<1, 0, 0, 0>())))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - carry_8
        let constraint_quotient = ((carry_8_col261
            - ((((carry_7_col260
                + (((a2_limb_2_col135 + b2_limb_2_col183) - c2_limb_2_col231)
                    - (p2_limb_2_col28 * sub_p_bit_col252)))
                + (qm31_const::<512, 0, 0, 0>()
                    * (((a2_limb_3_col136 + b2_limb_3_col184) - c2_limb_3_col232)
                        - (p2_limb_3_col29 * sub_p_bit_col252))))
                + (qm31_const::<262144, 0, 0, 0>()
                    * (((a2_limb_4_col137 + b2_limb_4_col185) - c2_limb_4_col233)
                        - (p2_limb_4_col30 * sub_p_bit_col252))))
                * qm31_const::<16, 0, 0, 0>())))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - carry is 0 or 1 or -1.
        let constraint_quotient = ((carry_8_col261
            * ((carry_8_col261 * carry_8_col261) - qm31_const::<1, 0, 0, 0>())))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - carry_9
        let constraint_quotient = ((carry_9_col262
            - ((((carry_8_col261
                + (((a2_limb_5_col138 + b2_limb_5_col186) - c2_limb_5_col234)
                    - (p2_limb_5_col31 * sub_p_bit_col252)))
                + (qm31_const::<512, 0, 0, 0>()
                    * (((a2_limb_6_col139 + b2_limb_6_col187) - c2_limb_6_col235)
                        - (p2_limb_6_col32 * sub_p_bit_col252))))
                + (qm31_const::<262144, 0, 0, 0>()
                    * (((a2_limb_7_col140 + b2_limb_7_col188) - c2_limb_7_col236)
                        - (p2_limb_7_col33 * sub_p_bit_col252))))
                * qm31_const::<16, 0, 0, 0>())))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - carry is 0 or 1 or -1.
        let constraint_quotient = ((carry_9_col262
            * ((carry_9_col262 * carry_9_col262) - qm31_const::<1, 0, 0, 0>())))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - carry_10
        let constraint_quotient = ((carry_10_col263
            - ((((carry_9_col262
                + (((a2_limb_8_col141 + b2_limb_8_col189) - c2_limb_8_col237)
                    - (p2_limb_8_col34 * sub_p_bit_col252)))
                + (qm31_const::<512, 0, 0, 0>()
                    * (((a2_limb_9_col142 + b2_limb_9_col190) - c2_limb_9_col238)
                        - (p2_limb_9_col35 * sub_p_bit_col252))))
                + (qm31_const::<262144, 0, 0, 0>()
                    * (((a2_limb_10_col143 + b2_limb_10_col191) - c2_limb_10_col239)
                        - (p2_limb_10_col36 * sub_p_bit_col252))))
                * qm31_const::<128, 0, 0, 0>())))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - carry is 0 or 1 or -1.
        let constraint_quotient = ((carry_10_col263
            * ((carry_10_col263 * carry_10_col263) - qm31_const::<1, 0, 0, 0>())))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - carry_11
        let constraint_quotient = ((carry_11_col264
            - ((((carry_10_col263
                + (((a3_limb_0_col145 + b3_limb_0_col193) - c3_limb_0_col241)
                    - (p3_limb_0_col38 * sub_p_bit_col252)))
                + (qm31_const::<512, 0, 0, 0>()
                    * (((a3_limb_1_col146 + b3_limb_1_col194) - c3_limb_1_col242)
                        - (p3_limb_1_col39 * sub_p_bit_col252))))
                + (qm31_const::<262144, 0, 0, 0>()
                    * (((a3_limb_2_col147 + b3_limb_2_col195) - c3_limb_2_col243)
                        - (p3_limb_2_col40 * sub_p_bit_col252))))
                * qm31_const::<16, 0, 0, 0>())))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - carry is 0 or 1 or -1.
        let constraint_quotient = ((carry_11_col264
            * ((carry_11_col264 * carry_11_col264) - qm31_const::<1, 0, 0, 0>())))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - carry_12
        let constraint_quotient = ((carry_12_col265
            - ((((carry_11_col264
                + (((a3_limb_3_col148 + b3_limb_3_col196) - c3_limb_3_col244)
                    - (p3_limb_3_col41 * sub_p_bit_col252)))
                + (qm31_const::<512, 0, 0, 0>()
                    * (((a3_limb_4_col149 + b3_limb_4_col197) - c3_limb_4_col245)
                        - (p3_limb_4_col42 * sub_p_bit_col252))))
                + (qm31_const::<262144, 0, 0, 0>()
                    * (((a3_limb_5_col150 + b3_limb_5_col198) - c3_limb_5_col246)
                        - (p3_limb_5_col43 * sub_p_bit_col252))))
                * qm31_const::<16, 0, 0, 0>())))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - carry is 0 or 1 or -1.
        let constraint_quotient = ((carry_12_col265
            * ((carry_12_col265 * carry_12_col265) - qm31_const::<1, 0, 0, 0>())))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - carry_13
        let constraint_quotient = ((carry_13_col266
            - ((((carry_12_col265
                + (((a3_limb_6_col151 + b3_limb_6_col199) - c3_limb_6_col247)
                    - (p3_limb_6_col44 * sub_p_bit_col252)))
                + (qm31_const::<512, 0, 0, 0>()
                    * (((a3_limb_7_col152 + b3_limb_7_col200) - c3_limb_7_col248)
                        - (p3_limb_7_col45 * sub_p_bit_col252))))
                + (qm31_const::<262144, 0, 0, 0>()
                    * (((a3_limb_8_col153 + b3_limb_8_col201) - c3_limb_8_col249)
                        - (p3_limb_8_col46 * sub_p_bit_col252))))
                * qm31_const::<16, 0, 0, 0>())))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - carry is 0 or 1 or -1.
        let constraint_quotient = ((carry_13_col266
            * ((carry_13_col266 * carry_13_col266) - qm31_const::<1, 0, 0, 0>())))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - last carry needs to be 0.
        let constraint_quotient = (((carry_13_col266
            + (((a3_limb_9_col154 + b3_limb_9_col202) - c3_limb_9_col250)
                - (p3_limb_9_col47 * sub_p_bit_col252)))
            + (qm31_const::<512, 0, 0, 0>()
                * (((a3_limb_10_col155 + b3_limb_10_col203) - c3_limb_10_col251)
                    - (p3_limb_10_col48 * sub_p_bit_col252)))))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        lookup_constraints(
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
            claimed_sum,
            column_size,
            ref interaction_trace_mask_values,
            memory_address_to_id_sum_0,
            memory_id_to_big_sum_1,
            memory_address_to_id_sum_2,
            memory_id_to_big_sum_3,
            memory_address_to_id_sum_4,
            memory_id_to_big_sum_5,
            memory_address_to_id_sum_6,
            memory_id_to_big_sum_7,
            memory_address_to_id_sum_8,
            memory_id_to_big_sum_9,
            memory_address_to_id_sum_10,
            memory_id_to_big_sum_11,
            memory_address_to_id_sum_12,
            memory_id_to_big_sum_13,
            memory_address_to_id_sum_14,
            memory_id_to_big_sum_15,
            memory_address_to_id_sum_16,
            memory_id_to_big_sum_17,
            memory_address_to_id_sum_18,
            memory_address_to_id_sum_19,
            memory_address_to_id_sum_20,
            memory_address_to_id_sum_21,
            memory_address_to_id_sum_22,
            memory_address_to_id_sum_23,
            memory_id_to_big_sum_24,
            memory_address_to_id_sum_25,
            memory_id_to_big_sum_26,
            memory_address_to_id_sum_27,
            memory_id_to_big_sum_28,
            memory_address_to_id_sum_29,
            memory_id_to_big_sum_30,
            memory_address_to_id_sum_31,
            memory_id_to_big_sum_32,
            memory_address_to_id_sum_33,
            memory_id_to_big_sum_34,
            memory_address_to_id_sum_35,
            memory_id_to_big_sum_36,
            memory_address_to_id_sum_37,
            memory_id_to_big_sum_38,
            memory_address_to_id_sum_39,
            memory_id_to_big_sum_40,
            memory_address_to_id_sum_41,
            memory_id_to_big_sum_42,
            memory_address_to_id_sum_43,
            memory_id_to_big_sum_44,
            memory_address_to_id_sum_45,
            memory_id_to_big_sum_46,
            memory_address_to_id_sum_47,
            memory_id_to_big_sum_48,
            memory_address_to_id_sum_49,
            memory_id_to_big_sum_50,
            memory_address_to_id_sum_51,
            memory_id_to_big_sum_52,
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
    memory_address_to_id_sum_0: QM31,
    memory_id_to_big_sum_1: QM31,
    memory_address_to_id_sum_2: QM31,
    memory_id_to_big_sum_3: QM31,
    memory_address_to_id_sum_4: QM31,
    memory_id_to_big_sum_5: QM31,
    memory_address_to_id_sum_6: QM31,
    memory_id_to_big_sum_7: QM31,
    memory_address_to_id_sum_8: QM31,
    memory_id_to_big_sum_9: QM31,
    memory_address_to_id_sum_10: QM31,
    memory_id_to_big_sum_11: QM31,
    memory_address_to_id_sum_12: QM31,
    memory_id_to_big_sum_13: QM31,
    memory_address_to_id_sum_14: QM31,
    memory_id_to_big_sum_15: QM31,
    memory_address_to_id_sum_16: QM31,
    memory_id_to_big_sum_17: QM31,
    memory_address_to_id_sum_18: QM31,
    memory_address_to_id_sum_19: QM31,
    memory_address_to_id_sum_20: QM31,
    memory_address_to_id_sum_21: QM31,
    memory_address_to_id_sum_22: QM31,
    memory_address_to_id_sum_23: QM31,
    memory_id_to_big_sum_24: QM31,
    memory_address_to_id_sum_25: QM31,
    memory_id_to_big_sum_26: QM31,
    memory_address_to_id_sum_27: QM31,
    memory_id_to_big_sum_28: QM31,
    memory_address_to_id_sum_29: QM31,
    memory_id_to_big_sum_30: QM31,
    memory_address_to_id_sum_31: QM31,
    memory_id_to_big_sum_32: QM31,
    memory_address_to_id_sum_33: QM31,
    memory_id_to_big_sum_34: QM31,
    memory_address_to_id_sum_35: QM31,
    memory_id_to_big_sum_36: QM31,
    memory_address_to_id_sum_37: QM31,
    memory_id_to_big_sum_38: QM31,
    memory_address_to_id_sum_39: QM31,
    memory_id_to_big_sum_40: QM31,
    memory_address_to_id_sum_41: QM31,
    memory_id_to_big_sum_42: QM31,
    memory_address_to_id_sum_43: QM31,
    memory_id_to_big_sum_44: QM31,
    memory_address_to_id_sum_45: QM31,
    memory_id_to_big_sum_46: QM31,
    memory_address_to_id_sum_47: QM31,
    memory_id_to_big_sum_48: QM31,
    memory_address_to_id_sum_49: QM31,
    memory_id_to_big_sum_50: QM31,
    memory_address_to_id_sum_51: QM31,
    memory_id_to_big_sum_52: QM31,
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
        trace_2_col40,
        trace_2_col41,
        trace_2_col42,
        trace_2_col43,
        trace_2_col44,
        trace_2_col45,
        trace_2_col46,
        trace_2_col47,
        trace_2_col48,
        trace_2_col49,
        trace_2_col50,
        trace_2_col51,
        trace_2_col52,
        trace_2_col53,
        trace_2_col54,
        trace_2_col55,
        trace_2_col56,
        trace_2_col57,
        trace_2_col58,
        trace_2_col59,
        trace_2_col60,
        trace_2_col61,
        trace_2_col62,
        trace_2_col63,
        trace_2_col64,
        trace_2_col65,
        trace_2_col66,
        trace_2_col67,
        trace_2_col68,
        trace_2_col69,
        trace_2_col70,
        trace_2_col71,
        trace_2_col72,
        trace_2_col73,
        trace_2_col74,
        trace_2_col75,
        trace_2_col76,
        trace_2_col77,
        trace_2_col78,
        trace_2_col79,
        trace_2_col80,
        trace_2_col81,
        trace_2_col82,
        trace_2_col83,
        trace_2_col84,
        trace_2_col85,
        trace_2_col86,
        trace_2_col87,
        trace_2_col88,
        trace_2_col89,
        trace_2_col90,
        trace_2_col91,
        trace_2_col92,
        trace_2_col93,
        trace_2_col94,
        trace_2_col95,
        trace_2_col96,
        trace_2_col97,
        trace_2_col98,
        trace_2_col99,
        trace_2_col100,
        trace_2_col101,
        trace_2_col102,
        trace_2_col103,
        trace_2_col104,
        trace_2_col105,
        trace_2_col106,
        trace_2_col107,
    ]: [Span<QM31>; 108] =
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
    let [trace_2_col36]: [QM31; 1] = (*trace_2_col36.try_into().unwrap()).unbox();
    let [trace_2_col37]: [QM31; 1] = (*trace_2_col37.try_into().unwrap()).unbox();
    let [trace_2_col38]: [QM31; 1] = (*trace_2_col38.try_into().unwrap()).unbox();
    let [trace_2_col39]: [QM31; 1] = (*trace_2_col39.try_into().unwrap()).unbox();
    let [trace_2_col40]: [QM31; 1] = (*trace_2_col40.try_into().unwrap()).unbox();
    let [trace_2_col41]: [QM31; 1] = (*trace_2_col41.try_into().unwrap()).unbox();
    let [trace_2_col42]: [QM31; 1] = (*trace_2_col42.try_into().unwrap()).unbox();
    let [trace_2_col43]: [QM31; 1] = (*trace_2_col43.try_into().unwrap()).unbox();
    let [trace_2_col44]: [QM31; 1] = (*trace_2_col44.try_into().unwrap()).unbox();
    let [trace_2_col45]: [QM31; 1] = (*trace_2_col45.try_into().unwrap()).unbox();
    let [trace_2_col46]: [QM31; 1] = (*trace_2_col46.try_into().unwrap()).unbox();
    let [trace_2_col47]: [QM31; 1] = (*trace_2_col47.try_into().unwrap()).unbox();
    let [trace_2_col48]: [QM31; 1] = (*trace_2_col48.try_into().unwrap()).unbox();
    let [trace_2_col49]: [QM31; 1] = (*trace_2_col49.try_into().unwrap()).unbox();
    let [trace_2_col50]: [QM31; 1] = (*trace_2_col50.try_into().unwrap()).unbox();
    let [trace_2_col51]: [QM31; 1] = (*trace_2_col51.try_into().unwrap()).unbox();
    let [trace_2_col52]: [QM31; 1] = (*trace_2_col52.try_into().unwrap()).unbox();
    let [trace_2_col53]: [QM31; 1] = (*trace_2_col53.try_into().unwrap()).unbox();
    let [trace_2_col54]: [QM31; 1] = (*trace_2_col54.try_into().unwrap()).unbox();
    let [trace_2_col55]: [QM31; 1] = (*trace_2_col55.try_into().unwrap()).unbox();
    let [trace_2_col56]: [QM31; 1] = (*trace_2_col56.try_into().unwrap()).unbox();
    let [trace_2_col57]: [QM31; 1] = (*trace_2_col57.try_into().unwrap()).unbox();
    let [trace_2_col58]: [QM31; 1] = (*trace_2_col58.try_into().unwrap()).unbox();
    let [trace_2_col59]: [QM31; 1] = (*trace_2_col59.try_into().unwrap()).unbox();
    let [trace_2_col60]: [QM31; 1] = (*trace_2_col60.try_into().unwrap()).unbox();
    let [trace_2_col61]: [QM31; 1] = (*trace_2_col61.try_into().unwrap()).unbox();
    let [trace_2_col62]: [QM31; 1] = (*trace_2_col62.try_into().unwrap()).unbox();
    let [trace_2_col63]: [QM31; 1] = (*trace_2_col63.try_into().unwrap()).unbox();
    let [trace_2_col64]: [QM31; 1] = (*trace_2_col64.try_into().unwrap()).unbox();
    let [trace_2_col65]: [QM31; 1] = (*trace_2_col65.try_into().unwrap()).unbox();
    let [trace_2_col66]: [QM31; 1] = (*trace_2_col66.try_into().unwrap()).unbox();
    let [trace_2_col67]: [QM31; 1] = (*trace_2_col67.try_into().unwrap()).unbox();
    let [trace_2_col68]: [QM31; 1] = (*trace_2_col68.try_into().unwrap()).unbox();
    let [trace_2_col69]: [QM31; 1] = (*trace_2_col69.try_into().unwrap()).unbox();
    let [trace_2_col70]: [QM31; 1] = (*trace_2_col70.try_into().unwrap()).unbox();
    let [trace_2_col71]: [QM31; 1] = (*trace_2_col71.try_into().unwrap()).unbox();
    let [trace_2_col72]: [QM31; 1] = (*trace_2_col72.try_into().unwrap()).unbox();
    let [trace_2_col73]: [QM31; 1] = (*trace_2_col73.try_into().unwrap()).unbox();
    let [trace_2_col74]: [QM31; 1] = (*trace_2_col74.try_into().unwrap()).unbox();
    let [trace_2_col75]: [QM31; 1] = (*trace_2_col75.try_into().unwrap()).unbox();
    let [trace_2_col76]: [QM31; 1] = (*trace_2_col76.try_into().unwrap()).unbox();
    let [trace_2_col77]: [QM31; 1] = (*trace_2_col77.try_into().unwrap()).unbox();
    let [trace_2_col78]: [QM31; 1] = (*trace_2_col78.try_into().unwrap()).unbox();
    let [trace_2_col79]: [QM31; 1] = (*trace_2_col79.try_into().unwrap()).unbox();
    let [trace_2_col80]: [QM31; 1] = (*trace_2_col80.try_into().unwrap()).unbox();
    let [trace_2_col81]: [QM31; 1] = (*trace_2_col81.try_into().unwrap()).unbox();
    let [trace_2_col82]: [QM31; 1] = (*trace_2_col82.try_into().unwrap()).unbox();
    let [trace_2_col83]: [QM31; 1] = (*trace_2_col83.try_into().unwrap()).unbox();
    let [trace_2_col84]: [QM31; 1] = (*trace_2_col84.try_into().unwrap()).unbox();
    let [trace_2_col85]: [QM31; 1] = (*trace_2_col85.try_into().unwrap()).unbox();
    let [trace_2_col86]: [QM31; 1] = (*trace_2_col86.try_into().unwrap()).unbox();
    let [trace_2_col87]: [QM31; 1] = (*trace_2_col87.try_into().unwrap()).unbox();
    let [trace_2_col88]: [QM31; 1] = (*trace_2_col88.try_into().unwrap()).unbox();
    let [trace_2_col89]: [QM31; 1] = (*trace_2_col89.try_into().unwrap()).unbox();
    let [trace_2_col90]: [QM31; 1] = (*trace_2_col90.try_into().unwrap()).unbox();
    let [trace_2_col91]: [QM31; 1] = (*trace_2_col91.try_into().unwrap()).unbox();
    let [trace_2_col92]: [QM31; 1] = (*trace_2_col92.try_into().unwrap()).unbox();
    let [trace_2_col93]: [QM31; 1] = (*trace_2_col93.try_into().unwrap()).unbox();
    let [trace_2_col94]: [QM31; 1] = (*trace_2_col94.try_into().unwrap()).unbox();
    let [trace_2_col95]: [QM31; 1] = (*trace_2_col95.try_into().unwrap()).unbox();
    let [trace_2_col96]: [QM31; 1] = (*trace_2_col96.try_into().unwrap()).unbox();
    let [trace_2_col97]: [QM31; 1] = (*trace_2_col97.try_into().unwrap()).unbox();
    let [trace_2_col98]: [QM31; 1] = (*trace_2_col98.try_into().unwrap()).unbox();
    let [trace_2_col99]: [QM31; 1] = (*trace_2_col99.try_into().unwrap()).unbox();
    let [trace_2_col100]: [QM31; 1] = (*trace_2_col100.try_into().unwrap()).unbox();
    let [trace_2_col101]: [QM31; 1] = (*trace_2_col101.try_into().unwrap()).unbox();
    let [trace_2_col102]: [QM31; 1] = (*trace_2_col102.try_into().unwrap()).unbox();
    let [trace_2_col103]: [QM31; 1] = (*trace_2_col103.try_into().unwrap()).unbox();
    let [trace_2_col104_neg1, trace_2_col104]: [QM31; 2] = (*trace_2_col104.try_into().unwrap())
        .unbox();
    let [trace_2_col105_neg1, trace_2_col105]: [QM31; 2] = (*trace_2_col105.try_into().unwrap())
        .unbox();
    let [trace_2_col106_neg1, trace_2_col106]: [QM31; 2] = (*trace_2_col106.try_into().unwrap())
        .unbox();
    let [trace_2_col107_neg1, trace_2_col107]: [QM31; 2] = (*trace_2_col107.try_into().unwrap())
        .unbox();

    core::internal::revoke_ap_tracking();

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3],
    ))
        * memory_address_to_id_sum_0
        * memory_id_to_big_sum_1)
        - memory_address_to_id_sum_0
        - memory_id_to_big_sum_1)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7],
    )
        - QM31Impl::from_partial_evals([trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3]))
        * memory_address_to_id_sum_2
        * memory_id_to_big_sum_3)
        - memory_address_to_id_sum_2
        - memory_id_to_big_sum_3)
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
        * memory_address_to_id_sum_6
        * memory_id_to_big_sum_7)
        - memory_address_to_id_sum_6
        - memory_id_to_big_sum_7)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col16, trace_2_col17, trace_2_col18, trace_2_col19],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15],
        ))
        * memory_address_to_id_sum_8
        * memory_id_to_big_sum_9)
        - memory_address_to_id_sum_8
        - memory_id_to_big_sum_9)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col20, trace_2_col21, trace_2_col22, trace_2_col23],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col16, trace_2_col17, trace_2_col18, trace_2_col19],
        ))
        * memory_address_to_id_sum_10
        * memory_id_to_big_sum_11)
        - memory_address_to_id_sum_10
        - memory_id_to_big_sum_11)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col24, trace_2_col25, trace_2_col26, trace_2_col27],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col20, trace_2_col21, trace_2_col22, trace_2_col23],
        ))
        * memory_address_to_id_sum_12
        * memory_id_to_big_sum_13)
        - memory_address_to_id_sum_12
        - memory_id_to_big_sum_13)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col28, trace_2_col29, trace_2_col30, trace_2_col31],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col24, trace_2_col25, trace_2_col26, trace_2_col27],
        ))
        * memory_address_to_id_sum_14
        * memory_id_to_big_sum_15)
        - memory_address_to_id_sum_14
        - memory_id_to_big_sum_15)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col32, trace_2_col33, trace_2_col34, trace_2_col35],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col28, trace_2_col29, trace_2_col30, trace_2_col31],
        ))
        * memory_address_to_id_sum_16
        * memory_id_to_big_sum_17)
        - memory_address_to_id_sum_16
        - memory_id_to_big_sum_17)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col36, trace_2_col37, trace_2_col38, trace_2_col39],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col32, trace_2_col33, trace_2_col34, trace_2_col35],
        ))
        * memory_address_to_id_sum_18
        * memory_address_to_id_sum_19)
        - memory_address_to_id_sum_18
        - memory_address_to_id_sum_19)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col40, trace_2_col41, trace_2_col42, trace_2_col43],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col36, trace_2_col37, trace_2_col38, trace_2_col39],
        ))
        * memory_address_to_id_sum_20
        * memory_address_to_id_sum_21)
        - memory_address_to_id_sum_20
        - memory_address_to_id_sum_21)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col44, trace_2_col45, trace_2_col46, trace_2_col47],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col40, trace_2_col41, trace_2_col42, trace_2_col43],
        ))
        * memory_address_to_id_sum_22
        * memory_address_to_id_sum_23)
        - memory_address_to_id_sum_22
        - memory_address_to_id_sum_23)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col48, trace_2_col49, trace_2_col50, trace_2_col51],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col44, trace_2_col45, trace_2_col46, trace_2_col47],
        ))
        * memory_id_to_big_sum_24
        * memory_address_to_id_sum_25)
        - memory_id_to_big_sum_24
        - memory_address_to_id_sum_25)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col52, trace_2_col53, trace_2_col54, trace_2_col55],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col48, trace_2_col49, trace_2_col50, trace_2_col51],
        ))
        * memory_id_to_big_sum_26
        * memory_address_to_id_sum_27)
        - memory_id_to_big_sum_26
        - memory_address_to_id_sum_27)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col56, trace_2_col57, trace_2_col58, trace_2_col59],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col52, trace_2_col53, trace_2_col54, trace_2_col55],
        ))
        * memory_id_to_big_sum_28
        * memory_address_to_id_sum_29)
        - memory_id_to_big_sum_28
        - memory_address_to_id_sum_29)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col60, trace_2_col61, trace_2_col62, trace_2_col63],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col56, trace_2_col57, trace_2_col58, trace_2_col59],
        ))
        * memory_id_to_big_sum_30
        * memory_address_to_id_sum_31)
        - memory_id_to_big_sum_30
        - memory_address_to_id_sum_31)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col64, trace_2_col65, trace_2_col66, trace_2_col67],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col60, trace_2_col61, trace_2_col62, trace_2_col63],
        ))
        * memory_id_to_big_sum_32
        * memory_address_to_id_sum_33)
        - memory_id_to_big_sum_32
        - memory_address_to_id_sum_33)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col68, trace_2_col69, trace_2_col70, trace_2_col71],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col64, trace_2_col65, trace_2_col66, trace_2_col67],
        ))
        * memory_id_to_big_sum_34
        * memory_address_to_id_sum_35)
        - memory_id_to_big_sum_34
        - memory_address_to_id_sum_35)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col72, trace_2_col73, trace_2_col74, trace_2_col75],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col68, trace_2_col69, trace_2_col70, trace_2_col71],
        ))
        * memory_id_to_big_sum_36
        * memory_address_to_id_sum_37)
        - memory_id_to_big_sum_36
        - memory_address_to_id_sum_37)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col76, trace_2_col77, trace_2_col78, trace_2_col79],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col72, trace_2_col73, trace_2_col74, trace_2_col75],
        ))
        * memory_id_to_big_sum_38
        * memory_address_to_id_sum_39)
        - memory_id_to_big_sum_38
        - memory_address_to_id_sum_39)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col80, trace_2_col81, trace_2_col82, trace_2_col83],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col76, trace_2_col77, trace_2_col78, trace_2_col79],
        ))
        * memory_id_to_big_sum_40
        * memory_address_to_id_sum_41)
        - memory_id_to_big_sum_40
        - memory_address_to_id_sum_41)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col84, trace_2_col85, trace_2_col86, trace_2_col87],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col80, trace_2_col81, trace_2_col82, trace_2_col83],
        ))
        * memory_id_to_big_sum_42
        * memory_address_to_id_sum_43)
        - memory_id_to_big_sum_42
        - memory_address_to_id_sum_43)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col88, trace_2_col89, trace_2_col90, trace_2_col91],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col84, trace_2_col85, trace_2_col86, trace_2_col87],
        ))
        * memory_id_to_big_sum_44
        * memory_address_to_id_sum_45)
        - memory_id_to_big_sum_44
        - memory_address_to_id_sum_45)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col92, trace_2_col93, trace_2_col94, trace_2_col95],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col88, trace_2_col89, trace_2_col90, trace_2_col91],
        ))
        * memory_id_to_big_sum_46
        * memory_address_to_id_sum_47)
        - memory_id_to_big_sum_46
        - memory_address_to_id_sum_47)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col96, trace_2_col97, trace_2_col98, trace_2_col99],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col92, trace_2_col93, trace_2_col94, trace_2_col95],
        ))
        * memory_id_to_big_sum_48
        * memory_address_to_id_sum_49)
        - memory_id_to_big_sum_48
        - memory_address_to_id_sum_49)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col100, trace_2_col101, trace_2_col102, trace_2_col103],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col96, trace_2_col97, trace_2_col98, trace_2_col99],
        ))
        * memory_id_to_big_sum_50
        * memory_address_to_id_sum_51)
        - memory_id_to_big_sum_50
        - memory_address_to_id_sum_51)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col104, trace_2_col105, trace_2_col106, trace_2_col107],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col100, trace_2_col101, trace_2_col102, trace_2_col103],
        )
        - QM31Impl::from_partial_evals(
            [trace_2_col104_neg1, trace_2_col105_neg1, trace_2_col106_neg1, trace_2_col107_neg1],
        )
        + (claimed_sum * (column_size.inverse().into())))
        * memory_id_to_big_sum_52)
        - qm31_const::<1, 0, 0, 0>())
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
}
