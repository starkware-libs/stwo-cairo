// AIR version 98896da1-dirty
use crate::prelude::*;use crate::components::subroutines::read_blake_word::read_blake_word_evaluate;use crate::components::subroutines::verify_blake_word::verify_blake_word_evaluate;

pub const N_TRACE_COLUMNS: usize = 176;pub const RELATION_USES_PER_ROW: [(felt252, u32); 4] = [
    ('RangeCheck_7_2_5', 24), ('MemoryAddressToId', 24), ('MemoryIdToBig', 24), ('Sha256Round', 1)
];

#[derive(Drop, Serde, Copy)]
pub struct Claim {
    pub log_size: u32,pub sha256_builtin_segment_start: u32,
}

pub impl ClaimImpl of ClaimTrait<Claim> {
    fn log_sizes(self: @Claim) -> TreeArray<Span<u32>> {
        let log_size = *(self.log_size);
        let preprocessed_log_sizes = array![log_size].span();
        let trace_log_sizes = [log_size; N_TRACE_COLUMNS].span();
        let interaction_log_sizes = [log_size; 148].span();
        array![preprocessed_log_sizes, trace_log_sizes, interaction_log_sizes]
    }

    fn mix_into(self: @Claim, ref channel: Channel) {
        channel.mix_u64((*(self.log_size)).into());channel.mix_u64((*self.sha256_builtin_segment_start).into());
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
    pub range_check_7_2_5_lookup_elements: crate::RangeCheck_7_2_5Elements,
pub memory_address_to_id_lookup_elements: crate::MemoryAddressToIdElements,
pub memory_id_to_big_lookup_elements: crate::MemoryIdToBigElements,
pub sha_256_round_lookup_elements: crate::Sha256RoundElements,
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
            range_check_7_2_5_lookup_elements: interaction_elements.range_checks.rc_7_2_5.clone(),
memory_address_to_id_lookup_elements: interaction_elements.memory_address_to_id.clone(),
memory_id_to_big_lookup_elements: interaction_elements.memory_id_to_value.clone(),
sha_256_round_lookup_elements: interaction_elements.sha_256_round.clone(),
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
        preprocessed_column_set.insert(PreprocessedColumn::Seq(*(self.claim.log_size)));trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
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
        let sha256_builtin_segment_start: QM31 = (TryInto::<u32, M31>::try_into((*(self.claim.sha256_builtin_segment_start))).unwrap()).into();let mut range_check_7_2_5_sum_0: QM31 = Zero::zero();let mut memory_address_to_id_sum_1: QM31 = Zero::zero();let mut memory_id_to_big_sum_2: QM31 = Zero::zero();let mut range_check_7_2_5_sum_3: QM31 = Zero::zero();let mut memory_address_to_id_sum_4: QM31 = Zero::zero();let mut memory_id_to_big_sum_5: QM31 = Zero::zero();let mut range_check_7_2_5_sum_6: QM31 = Zero::zero();let mut memory_address_to_id_sum_7: QM31 = Zero::zero();let mut memory_id_to_big_sum_8: QM31 = Zero::zero();let mut range_check_7_2_5_sum_9: QM31 = Zero::zero();let mut memory_address_to_id_sum_10: QM31 = Zero::zero();let mut memory_id_to_big_sum_11: QM31 = Zero::zero();let mut range_check_7_2_5_sum_12: QM31 = Zero::zero();let mut memory_address_to_id_sum_13: QM31 = Zero::zero();let mut memory_id_to_big_sum_14: QM31 = Zero::zero();let mut range_check_7_2_5_sum_15: QM31 = Zero::zero();let mut memory_address_to_id_sum_16: QM31 = Zero::zero();let mut memory_id_to_big_sum_17: QM31 = Zero::zero();let mut range_check_7_2_5_sum_18: QM31 = Zero::zero();let mut memory_address_to_id_sum_19: QM31 = Zero::zero();let mut memory_id_to_big_sum_20: QM31 = Zero::zero();let mut range_check_7_2_5_sum_21: QM31 = Zero::zero();let mut memory_address_to_id_sum_22: QM31 = Zero::zero();let mut memory_id_to_big_sum_23: QM31 = Zero::zero();let mut range_check_7_2_5_sum_24: QM31 = Zero::zero();let mut memory_address_to_id_sum_25: QM31 = Zero::zero();let mut memory_id_to_big_sum_26: QM31 = Zero::zero();let mut range_check_7_2_5_sum_27: QM31 = Zero::zero();let mut memory_address_to_id_sum_28: QM31 = Zero::zero();let mut memory_id_to_big_sum_29: QM31 = Zero::zero();let mut range_check_7_2_5_sum_30: QM31 = Zero::zero();let mut memory_address_to_id_sum_31: QM31 = Zero::zero();let mut memory_id_to_big_sum_32: QM31 = Zero::zero();let mut range_check_7_2_5_sum_33: QM31 = Zero::zero();let mut memory_address_to_id_sum_34: QM31 = Zero::zero();let mut memory_id_to_big_sum_35: QM31 = Zero::zero();let mut range_check_7_2_5_sum_36: QM31 = Zero::zero();let mut memory_address_to_id_sum_37: QM31 = Zero::zero();let mut memory_id_to_big_sum_38: QM31 = Zero::zero();let mut range_check_7_2_5_sum_39: QM31 = Zero::zero();let mut memory_address_to_id_sum_40: QM31 = Zero::zero();let mut memory_id_to_big_sum_41: QM31 = Zero::zero();let mut range_check_7_2_5_sum_42: QM31 = Zero::zero();let mut memory_address_to_id_sum_43: QM31 = Zero::zero();let mut memory_id_to_big_sum_44: QM31 = Zero::zero();let mut range_check_7_2_5_sum_45: QM31 = Zero::zero();let mut memory_address_to_id_sum_46: QM31 = Zero::zero();let mut memory_id_to_big_sum_47: QM31 = Zero::zero();let mut sha_256_round_sum_48: QM31 = Zero::zero();let mut sha_256_round_sum_49: QM31 = Zero::zero();let mut range_check_7_2_5_sum_50: QM31 = Zero::zero();let mut memory_address_to_id_sum_51: QM31 = Zero::zero();let mut memory_id_to_big_sum_52: QM31 = Zero::zero();let mut range_check_7_2_5_sum_53: QM31 = Zero::zero();let mut memory_address_to_id_sum_54: QM31 = Zero::zero();let mut memory_id_to_big_sum_55: QM31 = Zero::zero();let mut range_check_7_2_5_sum_56: QM31 = Zero::zero();let mut memory_address_to_id_sum_57: QM31 = Zero::zero();let mut memory_id_to_big_sum_58: QM31 = Zero::zero();let mut range_check_7_2_5_sum_59: QM31 = Zero::zero();let mut memory_address_to_id_sum_60: QM31 = Zero::zero();let mut memory_id_to_big_sum_61: QM31 = Zero::zero();let mut range_check_7_2_5_sum_62: QM31 = Zero::zero();let mut memory_address_to_id_sum_63: QM31 = Zero::zero();let mut memory_id_to_big_sum_64: QM31 = Zero::zero();let mut range_check_7_2_5_sum_65: QM31 = Zero::zero();let mut memory_address_to_id_sum_66: QM31 = Zero::zero();let mut memory_id_to_big_sum_67: QM31 = Zero::zero();let mut range_check_7_2_5_sum_68: QM31 = Zero::zero();let mut memory_address_to_id_sum_69: QM31 = Zero::zero();let mut memory_id_to_big_sum_70: QM31 = Zero::zero();let mut range_check_7_2_5_sum_71: QM31 = Zero::zero();let mut memory_address_to_id_sum_72: QM31 = Zero::zero();let mut memory_id_to_big_sum_73: QM31 = Zero::zero();let seq
            = preprocessed_mask_values.get(PreprocessedColumn::Seq(*(self.claim.log_size)));
        

        let [low_16_bits_col0, high_16_bits_col1, low_7_ms_bits_col2, high_14_ms_bits_col3, high_5_ms_bits_col4, input_0_id_col5, low_16_bits_col6, high_16_bits_col7, low_7_ms_bits_col8, high_14_ms_bits_col9, high_5_ms_bits_col10, input_1_id_col11, low_16_bits_col12, high_16_bits_col13, low_7_ms_bits_col14, high_14_ms_bits_col15, high_5_ms_bits_col16, input_2_id_col17, low_16_bits_col18, high_16_bits_col19, low_7_ms_bits_col20, high_14_ms_bits_col21, high_5_ms_bits_col22, input_3_id_col23, low_16_bits_col24, high_16_bits_col25, low_7_ms_bits_col26, high_14_ms_bits_col27, high_5_ms_bits_col28, input_4_id_col29, low_16_bits_col30, high_16_bits_col31, low_7_ms_bits_col32, high_14_ms_bits_col33, high_5_ms_bits_col34, input_5_id_col35, low_16_bits_col36, high_16_bits_col37, low_7_ms_bits_col38, high_14_ms_bits_col39, high_5_ms_bits_col40, input_6_id_col41, low_16_bits_col42, high_16_bits_col43, low_7_ms_bits_col44, high_14_ms_bits_col45, high_5_ms_bits_col46, input_7_id_col47, low_16_bits_col48, high_16_bits_col49, low_7_ms_bits_col50, high_14_ms_bits_col51, high_5_ms_bits_col52, input_8_id_col53, low_16_bits_col54, high_16_bits_col55, low_7_ms_bits_col56, high_14_ms_bits_col57, high_5_ms_bits_col58, input_9_id_col59, low_16_bits_col60, high_16_bits_col61, low_7_ms_bits_col62, high_14_ms_bits_col63, high_5_ms_bits_col64, input_10_id_col65, low_16_bits_col66, high_16_bits_col67, low_7_ms_bits_col68, high_14_ms_bits_col69, high_5_ms_bits_col70, input_11_id_col71, low_16_bits_col72, high_16_bits_col73, low_7_ms_bits_col74, high_14_ms_bits_col75, high_5_ms_bits_col76, input_12_id_col77, low_16_bits_col78, high_16_bits_col79, low_7_ms_bits_col80, high_14_ms_bits_col81, high_5_ms_bits_col82, input_13_id_col83, low_16_bits_col84, high_16_bits_col85, low_7_ms_bits_col86, high_14_ms_bits_col87, high_5_ms_bits_col88, input_14_id_col89, low_16_bits_col90, high_16_bits_col91, low_7_ms_bits_col92, high_14_ms_bits_col93, high_5_ms_bits_col94, input_15_id_col95, sha_256_round_output_limb_0_col96, sha_256_round_output_limb_1_col97, sha_256_round_output_limb_2_col98, sha_256_round_output_limb_3_col99, sha_256_round_output_limb_4_col100, sha_256_round_output_limb_5_col101, sha_256_round_output_limb_6_col102, sha_256_round_output_limb_7_col103, sha_256_round_output_limb_8_col104, sha_256_round_output_limb_9_col105, sha_256_round_output_limb_10_col106, sha_256_round_output_limb_11_col107, sha_256_round_output_limb_12_col108, sha_256_round_output_limb_13_col109, sha_256_round_output_limb_14_col110, sha_256_round_output_limb_15_col111, sha_256_round_output_limb_16_col112, sha_256_round_output_limb_17_col113, sha_256_round_output_limb_18_col114, sha_256_round_output_limb_19_col115, sha_256_round_output_limb_20_col116, sha_256_round_output_limb_21_col117, sha_256_round_output_limb_22_col118, sha_256_round_output_limb_23_col119, sha_256_round_output_limb_24_col120, sha_256_round_output_limb_25_col121, sha_256_round_output_limb_26_col122, sha_256_round_output_limb_27_col123, sha_256_round_output_limb_28_col124, sha_256_round_output_limb_29_col125, sha_256_round_output_limb_30_col126, sha_256_round_output_limb_31_col127, sha_256_round_output_limb_32_col128, sha_256_round_output_limb_33_col129, sha_256_round_output_limb_34_col130, sha_256_round_output_limb_35_col131, sha_256_round_output_limb_36_col132, sha_256_round_output_limb_37_col133, sha_256_round_output_limb_38_col134, sha_256_round_output_limb_39_col135, sha_256_round_output_limb_40_col136, sha_256_round_output_limb_41_col137, sha_256_round_output_limb_42_col138, sha_256_round_output_limb_43_col139, sha_256_round_output_limb_44_col140, sha_256_round_output_limb_45_col141, sha_256_round_output_limb_46_col142, sha_256_round_output_limb_47_col143, low_7_ms_bits_col144, high_14_ms_bits_col145, high_5_ms_bits_col146, output_0_id_col147, low_7_ms_bits_col148, high_14_ms_bits_col149, high_5_ms_bits_col150, output_1_id_col151, low_7_ms_bits_col152, high_14_ms_bits_col153, high_5_ms_bits_col154, output_2_id_col155, low_7_ms_bits_col156, high_14_ms_bits_col157, high_5_ms_bits_col158, output_3_id_col159, low_7_ms_bits_col160, high_14_ms_bits_col161, high_5_ms_bits_col162, output_4_id_col163, low_7_ms_bits_col164, high_14_ms_bits_col165, high_5_ms_bits_col166, output_5_id_col167, low_7_ms_bits_col168, high_14_ms_bits_col169, high_5_ms_bits_col170, output_6_id_col171, low_7_ms_bits_col172, high_14_ms_bits_col173, high_5_ms_bits_col174, output_7_id_col175]: [Span<QM31>; 176]
            = (*trace_mask_values.multi_pop_front().unwrap()).unbox();
        let [low_16_bits_col0]: [QM31; 1] = (*low_16_bits_col0.try_into().unwrap()).unbox();let [high_16_bits_col1]: [QM31; 1] = (*high_16_bits_col1.try_into().unwrap()).unbox();let [low_7_ms_bits_col2]: [QM31; 1] = (*low_7_ms_bits_col2.try_into().unwrap()).unbox();let [high_14_ms_bits_col3]: [QM31; 1] = (*high_14_ms_bits_col3.try_into().unwrap()).unbox();let [high_5_ms_bits_col4]: [QM31; 1] = (*high_5_ms_bits_col4.try_into().unwrap()).unbox();let [input_0_id_col5]: [QM31; 1] = (*input_0_id_col5.try_into().unwrap()).unbox();let [low_16_bits_col6]: [QM31; 1] = (*low_16_bits_col6.try_into().unwrap()).unbox();let [high_16_bits_col7]: [QM31; 1] = (*high_16_bits_col7.try_into().unwrap()).unbox();let [low_7_ms_bits_col8]: [QM31; 1] = (*low_7_ms_bits_col8.try_into().unwrap()).unbox();let [high_14_ms_bits_col9]: [QM31; 1] = (*high_14_ms_bits_col9.try_into().unwrap()).unbox();let [high_5_ms_bits_col10]: [QM31; 1] = (*high_5_ms_bits_col10.try_into().unwrap()).unbox();let [input_1_id_col11]: [QM31; 1] = (*input_1_id_col11.try_into().unwrap()).unbox();let [low_16_bits_col12]: [QM31; 1] = (*low_16_bits_col12.try_into().unwrap()).unbox();let [high_16_bits_col13]: [QM31; 1] = (*high_16_bits_col13.try_into().unwrap()).unbox();let [low_7_ms_bits_col14]: [QM31; 1] = (*low_7_ms_bits_col14.try_into().unwrap()).unbox();let [high_14_ms_bits_col15]: [QM31; 1] = (*high_14_ms_bits_col15.try_into().unwrap()).unbox();let [high_5_ms_bits_col16]: [QM31; 1] = (*high_5_ms_bits_col16.try_into().unwrap()).unbox();let [input_2_id_col17]: [QM31; 1] = (*input_2_id_col17.try_into().unwrap()).unbox();let [low_16_bits_col18]: [QM31; 1] = (*low_16_bits_col18.try_into().unwrap()).unbox();let [high_16_bits_col19]: [QM31; 1] = (*high_16_bits_col19.try_into().unwrap()).unbox();let [low_7_ms_bits_col20]: [QM31; 1] = (*low_7_ms_bits_col20.try_into().unwrap()).unbox();let [high_14_ms_bits_col21]: [QM31; 1] = (*high_14_ms_bits_col21.try_into().unwrap()).unbox();let [high_5_ms_bits_col22]: [QM31; 1] = (*high_5_ms_bits_col22.try_into().unwrap()).unbox();let [input_3_id_col23]: [QM31; 1] = (*input_3_id_col23.try_into().unwrap()).unbox();let [low_16_bits_col24]: [QM31; 1] = (*low_16_bits_col24.try_into().unwrap()).unbox();let [high_16_bits_col25]: [QM31; 1] = (*high_16_bits_col25.try_into().unwrap()).unbox();let [low_7_ms_bits_col26]: [QM31; 1] = (*low_7_ms_bits_col26.try_into().unwrap()).unbox();let [high_14_ms_bits_col27]: [QM31; 1] = (*high_14_ms_bits_col27.try_into().unwrap()).unbox();let [high_5_ms_bits_col28]: [QM31; 1] = (*high_5_ms_bits_col28.try_into().unwrap()).unbox();let [input_4_id_col29]: [QM31; 1] = (*input_4_id_col29.try_into().unwrap()).unbox();let [low_16_bits_col30]: [QM31; 1] = (*low_16_bits_col30.try_into().unwrap()).unbox();let [high_16_bits_col31]: [QM31; 1] = (*high_16_bits_col31.try_into().unwrap()).unbox();let [low_7_ms_bits_col32]: [QM31; 1] = (*low_7_ms_bits_col32.try_into().unwrap()).unbox();let [high_14_ms_bits_col33]: [QM31; 1] = (*high_14_ms_bits_col33.try_into().unwrap()).unbox();let [high_5_ms_bits_col34]: [QM31; 1] = (*high_5_ms_bits_col34.try_into().unwrap()).unbox();let [input_5_id_col35]: [QM31; 1] = (*input_5_id_col35.try_into().unwrap()).unbox();let [low_16_bits_col36]: [QM31; 1] = (*low_16_bits_col36.try_into().unwrap()).unbox();let [high_16_bits_col37]: [QM31; 1] = (*high_16_bits_col37.try_into().unwrap()).unbox();let [low_7_ms_bits_col38]: [QM31; 1] = (*low_7_ms_bits_col38.try_into().unwrap()).unbox();let [high_14_ms_bits_col39]: [QM31; 1] = (*high_14_ms_bits_col39.try_into().unwrap()).unbox();let [high_5_ms_bits_col40]: [QM31; 1] = (*high_5_ms_bits_col40.try_into().unwrap()).unbox();let [input_6_id_col41]: [QM31; 1] = (*input_6_id_col41.try_into().unwrap()).unbox();let [low_16_bits_col42]: [QM31; 1] = (*low_16_bits_col42.try_into().unwrap()).unbox();let [high_16_bits_col43]: [QM31; 1] = (*high_16_bits_col43.try_into().unwrap()).unbox();let [low_7_ms_bits_col44]: [QM31; 1] = (*low_7_ms_bits_col44.try_into().unwrap()).unbox();let [high_14_ms_bits_col45]: [QM31; 1] = (*high_14_ms_bits_col45.try_into().unwrap()).unbox();let [high_5_ms_bits_col46]: [QM31; 1] = (*high_5_ms_bits_col46.try_into().unwrap()).unbox();let [input_7_id_col47]: [QM31; 1] = (*input_7_id_col47.try_into().unwrap()).unbox();let [low_16_bits_col48]: [QM31; 1] = (*low_16_bits_col48.try_into().unwrap()).unbox();let [high_16_bits_col49]: [QM31; 1] = (*high_16_bits_col49.try_into().unwrap()).unbox();let [low_7_ms_bits_col50]: [QM31; 1] = (*low_7_ms_bits_col50.try_into().unwrap()).unbox();let [high_14_ms_bits_col51]: [QM31; 1] = (*high_14_ms_bits_col51.try_into().unwrap()).unbox();let [high_5_ms_bits_col52]: [QM31; 1] = (*high_5_ms_bits_col52.try_into().unwrap()).unbox();let [input_8_id_col53]: [QM31; 1] = (*input_8_id_col53.try_into().unwrap()).unbox();let [low_16_bits_col54]: [QM31; 1] = (*low_16_bits_col54.try_into().unwrap()).unbox();let [high_16_bits_col55]: [QM31; 1] = (*high_16_bits_col55.try_into().unwrap()).unbox();let [low_7_ms_bits_col56]: [QM31; 1] = (*low_7_ms_bits_col56.try_into().unwrap()).unbox();let [high_14_ms_bits_col57]: [QM31; 1] = (*high_14_ms_bits_col57.try_into().unwrap()).unbox();let [high_5_ms_bits_col58]: [QM31; 1] = (*high_5_ms_bits_col58.try_into().unwrap()).unbox();let [input_9_id_col59]: [QM31; 1] = (*input_9_id_col59.try_into().unwrap()).unbox();let [low_16_bits_col60]: [QM31; 1] = (*low_16_bits_col60.try_into().unwrap()).unbox();let [high_16_bits_col61]: [QM31; 1] = (*high_16_bits_col61.try_into().unwrap()).unbox();let [low_7_ms_bits_col62]: [QM31; 1] = (*low_7_ms_bits_col62.try_into().unwrap()).unbox();let [high_14_ms_bits_col63]: [QM31; 1] = (*high_14_ms_bits_col63.try_into().unwrap()).unbox();let [high_5_ms_bits_col64]: [QM31; 1] = (*high_5_ms_bits_col64.try_into().unwrap()).unbox();let [input_10_id_col65]: [QM31; 1] = (*input_10_id_col65.try_into().unwrap()).unbox();let [low_16_bits_col66]: [QM31; 1] = (*low_16_bits_col66.try_into().unwrap()).unbox();let [high_16_bits_col67]: [QM31; 1] = (*high_16_bits_col67.try_into().unwrap()).unbox();let [low_7_ms_bits_col68]: [QM31; 1] = (*low_7_ms_bits_col68.try_into().unwrap()).unbox();let [high_14_ms_bits_col69]: [QM31; 1] = (*high_14_ms_bits_col69.try_into().unwrap()).unbox();let [high_5_ms_bits_col70]: [QM31; 1] = (*high_5_ms_bits_col70.try_into().unwrap()).unbox();let [input_11_id_col71]: [QM31; 1] = (*input_11_id_col71.try_into().unwrap()).unbox();let [low_16_bits_col72]: [QM31; 1] = (*low_16_bits_col72.try_into().unwrap()).unbox();let [high_16_bits_col73]: [QM31; 1] = (*high_16_bits_col73.try_into().unwrap()).unbox();let [low_7_ms_bits_col74]: [QM31; 1] = (*low_7_ms_bits_col74.try_into().unwrap()).unbox();let [high_14_ms_bits_col75]: [QM31; 1] = (*high_14_ms_bits_col75.try_into().unwrap()).unbox();let [high_5_ms_bits_col76]: [QM31; 1] = (*high_5_ms_bits_col76.try_into().unwrap()).unbox();let [input_12_id_col77]: [QM31; 1] = (*input_12_id_col77.try_into().unwrap()).unbox();let [low_16_bits_col78]: [QM31; 1] = (*low_16_bits_col78.try_into().unwrap()).unbox();let [high_16_bits_col79]: [QM31; 1] = (*high_16_bits_col79.try_into().unwrap()).unbox();let [low_7_ms_bits_col80]: [QM31; 1] = (*low_7_ms_bits_col80.try_into().unwrap()).unbox();let [high_14_ms_bits_col81]: [QM31; 1] = (*high_14_ms_bits_col81.try_into().unwrap()).unbox();let [high_5_ms_bits_col82]: [QM31; 1] = (*high_5_ms_bits_col82.try_into().unwrap()).unbox();let [input_13_id_col83]: [QM31; 1] = (*input_13_id_col83.try_into().unwrap()).unbox();let [low_16_bits_col84]: [QM31; 1] = (*low_16_bits_col84.try_into().unwrap()).unbox();let [high_16_bits_col85]: [QM31; 1] = (*high_16_bits_col85.try_into().unwrap()).unbox();let [low_7_ms_bits_col86]: [QM31; 1] = (*low_7_ms_bits_col86.try_into().unwrap()).unbox();let [high_14_ms_bits_col87]: [QM31; 1] = (*high_14_ms_bits_col87.try_into().unwrap()).unbox();let [high_5_ms_bits_col88]: [QM31; 1] = (*high_5_ms_bits_col88.try_into().unwrap()).unbox();let [input_14_id_col89]: [QM31; 1] = (*input_14_id_col89.try_into().unwrap()).unbox();let [low_16_bits_col90]: [QM31; 1] = (*low_16_bits_col90.try_into().unwrap()).unbox();let [high_16_bits_col91]: [QM31; 1] = (*high_16_bits_col91.try_into().unwrap()).unbox();let [low_7_ms_bits_col92]: [QM31; 1] = (*low_7_ms_bits_col92.try_into().unwrap()).unbox();let [high_14_ms_bits_col93]: [QM31; 1] = (*high_14_ms_bits_col93.try_into().unwrap()).unbox();let [high_5_ms_bits_col94]: [QM31; 1] = (*high_5_ms_bits_col94.try_into().unwrap()).unbox();let [input_15_id_col95]: [QM31; 1] = (*input_15_id_col95.try_into().unwrap()).unbox();let [sha_256_round_output_limb_0_col96]: [QM31; 1] = (*sha_256_round_output_limb_0_col96.try_into().unwrap()).unbox();let [sha_256_round_output_limb_1_col97]: [QM31; 1] = (*sha_256_round_output_limb_1_col97.try_into().unwrap()).unbox();let [sha_256_round_output_limb_2_col98]: [QM31; 1] = (*sha_256_round_output_limb_2_col98.try_into().unwrap()).unbox();let [sha_256_round_output_limb_3_col99]: [QM31; 1] = (*sha_256_round_output_limb_3_col99.try_into().unwrap()).unbox();let [sha_256_round_output_limb_4_col100]: [QM31; 1] = (*sha_256_round_output_limb_4_col100.try_into().unwrap()).unbox();let [sha_256_round_output_limb_5_col101]: [QM31; 1] = (*sha_256_round_output_limb_5_col101.try_into().unwrap()).unbox();let [sha_256_round_output_limb_6_col102]: [QM31; 1] = (*sha_256_round_output_limb_6_col102.try_into().unwrap()).unbox();let [sha_256_round_output_limb_7_col103]: [QM31; 1] = (*sha_256_round_output_limb_7_col103.try_into().unwrap()).unbox();let [sha_256_round_output_limb_8_col104]: [QM31; 1] = (*sha_256_round_output_limb_8_col104.try_into().unwrap()).unbox();let [sha_256_round_output_limb_9_col105]: [QM31; 1] = (*sha_256_round_output_limb_9_col105.try_into().unwrap()).unbox();let [sha_256_round_output_limb_10_col106]: [QM31; 1] = (*sha_256_round_output_limb_10_col106.try_into().unwrap()).unbox();let [sha_256_round_output_limb_11_col107]: [QM31; 1] = (*sha_256_round_output_limb_11_col107.try_into().unwrap()).unbox();let [sha_256_round_output_limb_12_col108]: [QM31; 1] = (*sha_256_round_output_limb_12_col108.try_into().unwrap()).unbox();let [sha_256_round_output_limb_13_col109]: [QM31; 1] = (*sha_256_round_output_limb_13_col109.try_into().unwrap()).unbox();let [sha_256_round_output_limb_14_col110]: [QM31; 1] = (*sha_256_round_output_limb_14_col110.try_into().unwrap()).unbox();let [sha_256_round_output_limb_15_col111]: [QM31; 1] = (*sha_256_round_output_limb_15_col111.try_into().unwrap()).unbox();let [sha_256_round_output_limb_16_col112]: [QM31; 1] = (*sha_256_round_output_limb_16_col112.try_into().unwrap()).unbox();let [sha_256_round_output_limb_17_col113]: [QM31; 1] = (*sha_256_round_output_limb_17_col113.try_into().unwrap()).unbox();let [sha_256_round_output_limb_18_col114]: [QM31; 1] = (*sha_256_round_output_limb_18_col114.try_into().unwrap()).unbox();let [sha_256_round_output_limb_19_col115]: [QM31; 1] = (*sha_256_round_output_limb_19_col115.try_into().unwrap()).unbox();let [sha_256_round_output_limb_20_col116]: [QM31; 1] = (*sha_256_round_output_limb_20_col116.try_into().unwrap()).unbox();let [sha_256_round_output_limb_21_col117]: [QM31; 1] = (*sha_256_round_output_limb_21_col117.try_into().unwrap()).unbox();let [sha_256_round_output_limb_22_col118]: [QM31; 1] = (*sha_256_round_output_limb_22_col118.try_into().unwrap()).unbox();let [sha_256_round_output_limb_23_col119]: [QM31; 1] = (*sha_256_round_output_limb_23_col119.try_into().unwrap()).unbox();let [sha_256_round_output_limb_24_col120]: [QM31; 1] = (*sha_256_round_output_limb_24_col120.try_into().unwrap()).unbox();let [sha_256_round_output_limb_25_col121]: [QM31; 1] = (*sha_256_round_output_limb_25_col121.try_into().unwrap()).unbox();let [sha_256_round_output_limb_26_col122]: [QM31; 1] = (*sha_256_round_output_limb_26_col122.try_into().unwrap()).unbox();let [sha_256_round_output_limb_27_col123]: [QM31; 1] = (*sha_256_round_output_limb_27_col123.try_into().unwrap()).unbox();let [sha_256_round_output_limb_28_col124]: [QM31; 1] = (*sha_256_round_output_limb_28_col124.try_into().unwrap()).unbox();let [sha_256_round_output_limb_29_col125]: [QM31; 1] = (*sha_256_round_output_limb_29_col125.try_into().unwrap()).unbox();let [sha_256_round_output_limb_30_col126]: [QM31; 1] = (*sha_256_round_output_limb_30_col126.try_into().unwrap()).unbox();let [sha_256_round_output_limb_31_col127]: [QM31; 1] = (*sha_256_round_output_limb_31_col127.try_into().unwrap()).unbox();let [sha_256_round_output_limb_32_col128]: [QM31; 1] = (*sha_256_round_output_limb_32_col128.try_into().unwrap()).unbox();let [sha_256_round_output_limb_33_col129]: [QM31; 1] = (*sha_256_round_output_limb_33_col129.try_into().unwrap()).unbox();let [sha_256_round_output_limb_34_col130]: [QM31; 1] = (*sha_256_round_output_limb_34_col130.try_into().unwrap()).unbox();let [sha_256_round_output_limb_35_col131]: [QM31; 1] = (*sha_256_round_output_limb_35_col131.try_into().unwrap()).unbox();let [sha_256_round_output_limb_36_col132]: [QM31; 1] = (*sha_256_round_output_limb_36_col132.try_into().unwrap()).unbox();let [sha_256_round_output_limb_37_col133]: [QM31; 1] = (*sha_256_round_output_limb_37_col133.try_into().unwrap()).unbox();let [sha_256_round_output_limb_38_col134]: [QM31; 1] = (*sha_256_round_output_limb_38_col134.try_into().unwrap()).unbox();let [sha_256_round_output_limb_39_col135]: [QM31; 1] = (*sha_256_round_output_limb_39_col135.try_into().unwrap()).unbox();let [sha_256_round_output_limb_40_col136]: [QM31; 1] = (*sha_256_round_output_limb_40_col136.try_into().unwrap()).unbox();let [sha_256_round_output_limb_41_col137]: [QM31; 1] = (*sha_256_round_output_limb_41_col137.try_into().unwrap()).unbox();let [sha_256_round_output_limb_42_col138]: [QM31; 1] = (*sha_256_round_output_limb_42_col138.try_into().unwrap()).unbox();let [sha_256_round_output_limb_43_col139]: [QM31; 1] = (*sha_256_round_output_limb_43_col139.try_into().unwrap()).unbox();let [sha_256_round_output_limb_44_col140]: [QM31; 1] = (*sha_256_round_output_limb_44_col140.try_into().unwrap()).unbox();let [sha_256_round_output_limb_45_col141]: [QM31; 1] = (*sha_256_round_output_limb_45_col141.try_into().unwrap()).unbox();let [sha_256_round_output_limb_46_col142]: [QM31; 1] = (*sha_256_round_output_limb_46_col142.try_into().unwrap()).unbox();let [sha_256_round_output_limb_47_col143]: [QM31; 1] = (*sha_256_round_output_limb_47_col143.try_into().unwrap()).unbox();let [low_7_ms_bits_col144]: [QM31; 1] = (*low_7_ms_bits_col144.try_into().unwrap()).unbox();let [high_14_ms_bits_col145]: [QM31; 1] = (*high_14_ms_bits_col145.try_into().unwrap()).unbox();let [high_5_ms_bits_col146]: [QM31; 1] = (*high_5_ms_bits_col146.try_into().unwrap()).unbox();let [output_0_id_col147]: [QM31; 1] = (*output_0_id_col147.try_into().unwrap()).unbox();let [low_7_ms_bits_col148]: [QM31; 1] = (*low_7_ms_bits_col148.try_into().unwrap()).unbox();let [high_14_ms_bits_col149]: [QM31; 1] = (*high_14_ms_bits_col149.try_into().unwrap()).unbox();let [high_5_ms_bits_col150]: [QM31; 1] = (*high_5_ms_bits_col150.try_into().unwrap()).unbox();let [output_1_id_col151]: [QM31; 1] = (*output_1_id_col151.try_into().unwrap()).unbox();let [low_7_ms_bits_col152]: [QM31; 1] = (*low_7_ms_bits_col152.try_into().unwrap()).unbox();let [high_14_ms_bits_col153]: [QM31; 1] = (*high_14_ms_bits_col153.try_into().unwrap()).unbox();let [high_5_ms_bits_col154]: [QM31; 1] = (*high_5_ms_bits_col154.try_into().unwrap()).unbox();let [output_2_id_col155]: [QM31; 1] = (*output_2_id_col155.try_into().unwrap()).unbox();let [low_7_ms_bits_col156]: [QM31; 1] = (*low_7_ms_bits_col156.try_into().unwrap()).unbox();let [high_14_ms_bits_col157]: [QM31; 1] = (*high_14_ms_bits_col157.try_into().unwrap()).unbox();let [high_5_ms_bits_col158]: [QM31; 1] = (*high_5_ms_bits_col158.try_into().unwrap()).unbox();let [output_3_id_col159]: [QM31; 1] = (*output_3_id_col159.try_into().unwrap()).unbox();let [low_7_ms_bits_col160]: [QM31; 1] = (*low_7_ms_bits_col160.try_into().unwrap()).unbox();let [high_14_ms_bits_col161]: [QM31; 1] = (*high_14_ms_bits_col161.try_into().unwrap()).unbox();let [high_5_ms_bits_col162]: [QM31; 1] = (*high_5_ms_bits_col162.try_into().unwrap()).unbox();let [output_4_id_col163]: [QM31; 1] = (*output_4_id_col163.try_into().unwrap()).unbox();let [low_7_ms_bits_col164]: [QM31; 1] = (*low_7_ms_bits_col164.try_into().unwrap()).unbox();let [high_14_ms_bits_col165]: [QM31; 1] = (*high_14_ms_bits_col165.try_into().unwrap()).unbox();let [high_5_ms_bits_col166]: [QM31; 1] = (*high_5_ms_bits_col166.try_into().unwrap()).unbox();let [output_5_id_col167]: [QM31; 1] = (*output_5_id_col167.try_into().unwrap()).unbox();let [low_7_ms_bits_col168]: [QM31; 1] = (*low_7_ms_bits_col168.try_into().unwrap()).unbox();let [high_14_ms_bits_col169]: [QM31; 1] = (*high_14_ms_bits_col169.try_into().unwrap()).unbox();let [high_5_ms_bits_col170]: [QM31; 1] = (*high_5_ms_bits_col170.try_into().unwrap()).unbox();let [output_6_id_col171]: [QM31; 1] = (*output_6_id_col171.try_into().unwrap()).unbox();let [low_7_ms_bits_col172]: [QM31; 1] = (*low_7_ms_bits_col172.try_into().unwrap()).unbox();let [high_14_ms_bits_col173]: [QM31; 1] = (*high_14_ms_bits_col173.try_into().unwrap()).unbox();let [high_5_ms_bits_col174]: [QM31; 1] = (*high_5_ms_bits_col174.try_into().unwrap()).unbox();let [output_7_id_col175]: [QM31; 1] = (*output_7_id_col175.try_into().unwrap()).unbox();


        core::internal::revoke_ap_tracking();

        read_blake_word_evaluate(
            (sha256_builtin_segment_start + (seq * qm31_const::<24, 0, 0, 0>())),
low_16_bits_col0,
high_16_bits_col1,
low_7_ms_bits_col2,
high_14_ms_bits_col3,
high_5_ms_bits_col4,
input_0_id_col5,
self.range_check_7_2_5_lookup_elements,
self.memory_address_to_id_lookup_elements,
self.memory_id_to_big_lookup_elements,
ref range_check_7_2_5_sum_0,
ref memory_address_to_id_sum_1,
ref memory_id_to_big_sum_2,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );read_blake_word_evaluate(
            ((sha256_builtin_segment_start + (seq * qm31_const::<24, 0, 0, 0>())) + qm31_const::<1, 0, 0, 0>()),
low_16_bits_col6,
high_16_bits_col7,
low_7_ms_bits_col8,
high_14_ms_bits_col9,
high_5_ms_bits_col10,
input_1_id_col11,
self.range_check_7_2_5_lookup_elements,
self.memory_address_to_id_lookup_elements,
self.memory_id_to_big_lookup_elements,
ref range_check_7_2_5_sum_3,
ref memory_address_to_id_sum_4,
ref memory_id_to_big_sum_5,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );read_blake_word_evaluate(
            ((sha256_builtin_segment_start + (seq * qm31_const::<24, 0, 0, 0>())) + qm31_const::<2, 0, 0, 0>()),
low_16_bits_col12,
high_16_bits_col13,
low_7_ms_bits_col14,
high_14_ms_bits_col15,
high_5_ms_bits_col16,
input_2_id_col17,
self.range_check_7_2_5_lookup_elements,
self.memory_address_to_id_lookup_elements,
self.memory_id_to_big_lookup_elements,
ref range_check_7_2_5_sum_6,
ref memory_address_to_id_sum_7,
ref memory_id_to_big_sum_8,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );read_blake_word_evaluate(
            ((sha256_builtin_segment_start + (seq * qm31_const::<24, 0, 0, 0>())) + qm31_const::<3, 0, 0, 0>()),
low_16_bits_col18,
high_16_bits_col19,
low_7_ms_bits_col20,
high_14_ms_bits_col21,
high_5_ms_bits_col22,
input_3_id_col23,
self.range_check_7_2_5_lookup_elements,
self.memory_address_to_id_lookup_elements,
self.memory_id_to_big_lookup_elements,
ref range_check_7_2_5_sum_9,
ref memory_address_to_id_sum_10,
ref memory_id_to_big_sum_11,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );read_blake_word_evaluate(
            ((sha256_builtin_segment_start + (seq * qm31_const::<24, 0, 0, 0>())) + qm31_const::<4, 0, 0, 0>()),
low_16_bits_col24,
high_16_bits_col25,
low_7_ms_bits_col26,
high_14_ms_bits_col27,
high_5_ms_bits_col28,
input_4_id_col29,
self.range_check_7_2_5_lookup_elements,
self.memory_address_to_id_lookup_elements,
self.memory_id_to_big_lookup_elements,
ref range_check_7_2_5_sum_12,
ref memory_address_to_id_sum_13,
ref memory_id_to_big_sum_14,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );read_blake_word_evaluate(
            ((sha256_builtin_segment_start + (seq * qm31_const::<24, 0, 0, 0>())) + qm31_const::<5, 0, 0, 0>()),
low_16_bits_col30,
high_16_bits_col31,
low_7_ms_bits_col32,
high_14_ms_bits_col33,
high_5_ms_bits_col34,
input_5_id_col35,
self.range_check_7_2_5_lookup_elements,
self.memory_address_to_id_lookup_elements,
self.memory_id_to_big_lookup_elements,
ref range_check_7_2_5_sum_15,
ref memory_address_to_id_sum_16,
ref memory_id_to_big_sum_17,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );read_blake_word_evaluate(
            ((sha256_builtin_segment_start + (seq * qm31_const::<24, 0, 0, 0>())) + qm31_const::<6, 0, 0, 0>()),
low_16_bits_col36,
high_16_bits_col37,
low_7_ms_bits_col38,
high_14_ms_bits_col39,
high_5_ms_bits_col40,
input_6_id_col41,
self.range_check_7_2_5_lookup_elements,
self.memory_address_to_id_lookup_elements,
self.memory_id_to_big_lookup_elements,
ref range_check_7_2_5_sum_18,
ref memory_address_to_id_sum_19,
ref memory_id_to_big_sum_20,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );read_blake_word_evaluate(
            ((sha256_builtin_segment_start + (seq * qm31_const::<24, 0, 0, 0>())) + qm31_const::<7, 0, 0, 0>()),
low_16_bits_col42,
high_16_bits_col43,
low_7_ms_bits_col44,
high_14_ms_bits_col45,
high_5_ms_bits_col46,
input_7_id_col47,
self.range_check_7_2_5_lookup_elements,
self.memory_address_to_id_lookup_elements,
self.memory_id_to_big_lookup_elements,
ref range_check_7_2_5_sum_21,
ref memory_address_to_id_sum_22,
ref memory_id_to_big_sum_23,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );read_blake_word_evaluate(
            ((sha256_builtin_segment_start + (seq * qm31_const::<24, 0, 0, 0>())) + qm31_const::<8, 0, 0, 0>()),
low_16_bits_col48,
high_16_bits_col49,
low_7_ms_bits_col50,
high_14_ms_bits_col51,
high_5_ms_bits_col52,
input_8_id_col53,
self.range_check_7_2_5_lookup_elements,
self.memory_address_to_id_lookup_elements,
self.memory_id_to_big_lookup_elements,
ref range_check_7_2_5_sum_24,
ref memory_address_to_id_sum_25,
ref memory_id_to_big_sum_26,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );read_blake_word_evaluate(
            ((sha256_builtin_segment_start + (seq * qm31_const::<24, 0, 0, 0>())) + qm31_const::<9, 0, 0, 0>()),
low_16_bits_col54,
high_16_bits_col55,
low_7_ms_bits_col56,
high_14_ms_bits_col57,
high_5_ms_bits_col58,
input_9_id_col59,
self.range_check_7_2_5_lookup_elements,
self.memory_address_to_id_lookup_elements,
self.memory_id_to_big_lookup_elements,
ref range_check_7_2_5_sum_27,
ref memory_address_to_id_sum_28,
ref memory_id_to_big_sum_29,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );read_blake_word_evaluate(
            ((sha256_builtin_segment_start + (seq * qm31_const::<24, 0, 0, 0>())) + qm31_const::<10, 0, 0, 0>()),
low_16_bits_col60,
high_16_bits_col61,
low_7_ms_bits_col62,
high_14_ms_bits_col63,
high_5_ms_bits_col64,
input_10_id_col65,
self.range_check_7_2_5_lookup_elements,
self.memory_address_to_id_lookup_elements,
self.memory_id_to_big_lookup_elements,
ref range_check_7_2_5_sum_30,
ref memory_address_to_id_sum_31,
ref memory_id_to_big_sum_32,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );read_blake_word_evaluate(
            ((sha256_builtin_segment_start + (seq * qm31_const::<24, 0, 0, 0>())) + qm31_const::<11, 0, 0, 0>()),
low_16_bits_col66,
high_16_bits_col67,
low_7_ms_bits_col68,
high_14_ms_bits_col69,
high_5_ms_bits_col70,
input_11_id_col71,
self.range_check_7_2_5_lookup_elements,
self.memory_address_to_id_lookup_elements,
self.memory_id_to_big_lookup_elements,
ref range_check_7_2_5_sum_33,
ref memory_address_to_id_sum_34,
ref memory_id_to_big_sum_35,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );read_blake_word_evaluate(
            ((sha256_builtin_segment_start + (seq * qm31_const::<24, 0, 0, 0>())) + qm31_const::<12, 0, 0, 0>()),
low_16_bits_col72,
high_16_bits_col73,
low_7_ms_bits_col74,
high_14_ms_bits_col75,
high_5_ms_bits_col76,
input_12_id_col77,
self.range_check_7_2_5_lookup_elements,
self.memory_address_to_id_lookup_elements,
self.memory_id_to_big_lookup_elements,
ref range_check_7_2_5_sum_36,
ref memory_address_to_id_sum_37,
ref memory_id_to_big_sum_38,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );read_blake_word_evaluate(
            ((sha256_builtin_segment_start + (seq * qm31_const::<24, 0, 0, 0>())) + qm31_const::<13, 0, 0, 0>()),
low_16_bits_col78,
high_16_bits_col79,
low_7_ms_bits_col80,
high_14_ms_bits_col81,
high_5_ms_bits_col82,
input_13_id_col83,
self.range_check_7_2_5_lookup_elements,
self.memory_address_to_id_lookup_elements,
self.memory_id_to_big_lookup_elements,
ref range_check_7_2_5_sum_39,
ref memory_address_to_id_sum_40,
ref memory_id_to_big_sum_41,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );read_blake_word_evaluate(
            ((sha256_builtin_segment_start + (seq * qm31_const::<24, 0, 0, 0>())) + qm31_const::<14, 0, 0, 0>()),
low_16_bits_col84,
high_16_bits_col85,
low_7_ms_bits_col86,
high_14_ms_bits_col87,
high_5_ms_bits_col88,
input_14_id_col89,
self.range_check_7_2_5_lookup_elements,
self.memory_address_to_id_lookup_elements,
self.memory_id_to_big_lookup_elements,
ref range_check_7_2_5_sum_42,
ref memory_address_to_id_sum_43,
ref memory_id_to_big_sum_44,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );read_blake_word_evaluate(
            ((sha256_builtin_segment_start + (seq * qm31_const::<24, 0, 0, 0>())) + qm31_const::<15, 0, 0, 0>()),
low_16_bits_col90,
high_16_bits_col91,
low_7_ms_bits_col92,
high_14_ms_bits_col93,
high_5_ms_bits_col94,
input_15_id_col95,
self.range_check_7_2_5_lookup_elements,
self.memory_address_to_id_lookup_elements,
self.memory_id_to_big_lookup_elements,
ref range_check_7_2_5_sum_45,
ref memory_address_to_id_sum_46,
ref memory_id_to_big_sum_47,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );

        sha_256_round_sum_48 = self.sha_256_round_lookup_elements.combine_qm31(
            [
                seq,
qm31_const::<0, 0, 0, 0>(),
qm31_const::<58983, 0, 0, 0>(),
qm31_const::<27145, 0, 0, 0>(),
qm31_const::<44677, 0, 0, 0>(),
qm31_const::<47975, 0, 0, 0>(),
qm31_const::<62322, 0, 0, 0>(),
qm31_const::<15470, 0, 0, 0>(),
qm31_const::<62778, 0, 0, 0>(),
qm31_const::<42319, 0, 0, 0>(),
qm31_const::<21119, 0, 0, 0>(),
qm31_const::<20750, 0, 0, 0>(),
qm31_const::<26764, 0, 0, 0>(),
qm31_const::<39685, 0, 0, 0>(),
qm31_const::<55723, 0, 0, 0>(),
qm31_const::<8067, 0, 0, 0>(),
qm31_const::<52505, 0, 0, 0>(),
qm31_const::<23520, 0, 0, 0>(),
low_16_bits_col0,
high_16_bits_col1,
low_16_bits_col6,
high_16_bits_col7,
low_16_bits_col12,
high_16_bits_col13,
low_16_bits_col18,
high_16_bits_col19,
low_16_bits_col24,
high_16_bits_col25,
low_16_bits_col30,
high_16_bits_col31,
low_16_bits_col36,
high_16_bits_col37,
low_16_bits_col42,
high_16_bits_col43,
low_16_bits_col48,
high_16_bits_col49,
low_16_bits_col54,
high_16_bits_col55,
low_16_bits_col60,
high_16_bits_col61,
low_16_bits_col66,
high_16_bits_col67,
low_16_bits_col72,
high_16_bits_col73,
low_16_bits_col78,
high_16_bits_col79,
low_16_bits_col84,
high_16_bits_col85,
low_16_bits_col90,
high_16_bits_col91
            ],
        );

        sha_256_round_sum_49 = self.sha_256_round_lookup_elements.combine_qm31(
            [
                seq,
qm31_const::<64, 0, 0, 0>(),
sha_256_round_output_limb_0_col96,
sha_256_round_output_limb_1_col97,
sha_256_round_output_limb_2_col98,
sha_256_round_output_limb_3_col99,
sha_256_round_output_limb_4_col100,
sha_256_round_output_limb_5_col101,
sha_256_round_output_limb_6_col102,
sha_256_round_output_limb_7_col103,
sha_256_round_output_limb_8_col104,
sha_256_round_output_limb_9_col105,
sha_256_round_output_limb_10_col106,
sha_256_round_output_limb_11_col107,
sha_256_round_output_limb_12_col108,
sha_256_round_output_limb_13_col109,
sha_256_round_output_limb_14_col110,
sha_256_round_output_limb_15_col111,
sha_256_round_output_limb_16_col112,
sha_256_round_output_limb_17_col113,
sha_256_round_output_limb_18_col114,
sha_256_round_output_limb_19_col115,
sha_256_round_output_limb_20_col116,
sha_256_round_output_limb_21_col117,
sha_256_round_output_limb_22_col118,
sha_256_round_output_limb_23_col119,
sha_256_round_output_limb_24_col120,
sha_256_round_output_limb_25_col121,
sha_256_round_output_limb_26_col122,
sha_256_round_output_limb_27_col123,
sha_256_round_output_limb_28_col124,
sha_256_round_output_limb_29_col125,
sha_256_round_output_limb_30_col126,
sha_256_round_output_limb_31_col127,
sha_256_round_output_limb_32_col128,
sha_256_round_output_limb_33_col129,
sha_256_round_output_limb_34_col130,
sha_256_round_output_limb_35_col131,
sha_256_round_output_limb_36_col132,
sha_256_round_output_limb_37_col133,
sha_256_round_output_limb_38_col134,
sha_256_round_output_limb_39_col135,
sha_256_round_output_limb_40_col136,
sha_256_round_output_limb_41_col137,
sha_256_round_output_limb_42_col138,
sha_256_round_output_limb_43_col139,
sha_256_round_output_limb_44_col140,
sha_256_round_output_limb_45_col141,
sha_256_round_output_limb_46_col142,
sha_256_round_output_limb_47_col143
            ],
        );verify_blake_word_evaluate(
            [((sha256_builtin_segment_start + (seq * qm31_const::<24, 0, 0, 0>())) + qm31_const::<16, 0, 0, 0>()), sha_256_round_output_limb_0_col96, sha_256_round_output_limb_1_col97],
low_7_ms_bits_col144,
high_14_ms_bits_col145,
high_5_ms_bits_col146,
output_0_id_col147,
self.range_check_7_2_5_lookup_elements,
self.memory_address_to_id_lookup_elements,
self.memory_id_to_big_lookup_elements,
ref range_check_7_2_5_sum_50,
ref memory_address_to_id_sum_51,
ref memory_id_to_big_sum_52,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );verify_blake_word_evaluate(
            [((sha256_builtin_segment_start + (seq * qm31_const::<24, 0, 0, 0>())) + qm31_const::<17, 0, 0, 0>()), sha_256_round_output_limb_2_col98, sha_256_round_output_limb_3_col99],
low_7_ms_bits_col148,
high_14_ms_bits_col149,
high_5_ms_bits_col150,
output_1_id_col151,
self.range_check_7_2_5_lookup_elements,
self.memory_address_to_id_lookup_elements,
self.memory_id_to_big_lookup_elements,
ref range_check_7_2_5_sum_53,
ref memory_address_to_id_sum_54,
ref memory_id_to_big_sum_55,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );verify_blake_word_evaluate(
            [((sha256_builtin_segment_start + (seq * qm31_const::<24, 0, 0, 0>())) + qm31_const::<18, 0, 0, 0>()), sha_256_round_output_limb_4_col100, sha_256_round_output_limb_5_col101],
low_7_ms_bits_col152,
high_14_ms_bits_col153,
high_5_ms_bits_col154,
output_2_id_col155,
self.range_check_7_2_5_lookup_elements,
self.memory_address_to_id_lookup_elements,
self.memory_id_to_big_lookup_elements,
ref range_check_7_2_5_sum_56,
ref memory_address_to_id_sum_57,
ref memory_id_to_big_sum_58,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );verify_blake_word_evaluate(
            [((sha256_builtin_segment_start + (seq * qm31_const::<24, 0, 0, 0>())) + qm31_const::<19, 0, 0, 0>()), sha_256_round_output_limb_6_col102, sha_256_round_output_limb_7_col103],
low_7_ms_bits_col156,
high_14_ms_bits_col157,
high_5_ms_bits_col158,
output_3_id_col159,
self.range_check_7_2_5_lookup_elements,
self.memory_address_to_id_lookup_elements,
self.memory_id_to_big_lookup_elements,
ref range_check_7_2_5_sum_59,
ref memory_address_to_id_sum_60,
ref memory_id_to_big_sum_61,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );verify_blake_word_evaluate(
            [((sha256_builtin_segment_start + (seq * qm31_const::<24, 0, 0, 0>())) + qm31_const::<20, 0, 0, 0>()), sha_256_round_output_limb_8_col104, sha_256_round_output_limb_9_col105],
low_7_ms_bits_col160,
high_14_ms_bits_col161,
high_5_ms_bits_col162,
output_4_id_col163,
self.range_check_7_2_5_lookup_elements,
self.memory_address_to_id_lookup_elements,
self.memory_id_to_big_lookup_elements,
ref range_check_7_2_5_sum_62,
ref memory_address_to_id_sum_63,
ref memory_id_to_big_sum_64,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );verify_blake_word_evaluate(
            [((sha256_builtin_segment_start + (seq * qm31_const::<24, 0, 0, 0>())) + qm31_const::<21, 0, 0, 0>()), sha_256_round_output_limb_10_col106, sha_256_round_output_limb_11_col107],
low_7_ms_bits_col164,
high_14_ms_bits_col165,
high_5_ms_bits_col166,
output_5_id_col167,
self.range_check_7_2_5_lookup_elements,
self.memory_address_to_id_lookup_elements,
self.memory_id_to_big_lookup_elements,
ref range_check_7_2_5_sum_65,
ref memory_address_to_id_sum_66,
ref memory_id_to_big_sum_67,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );verify_blake_word_evaluate(
            [((sha256_builtin_segment_start + (seq * qm31_const::<24, 0, 0, 0>())) + qm31_const::<22, 0, 0, 0>()), sha_256_round_output_limb_12_col108, sha_256_round_output_limb_13_col109],
low_7_ms_bits_col168,
high_14_ms_bits_col169,
high_5_ms_bits_col170,
output_6_id_col171,
self.range_check_7_2_5_lookup_elements,
self.memory_address_to_id_lookup_elements,
self.memory_id_to_big_lookup_elements,
ref range_check_7_2_5_sum_68,
ref memory_address_to_id_sum_69,
ref memory_id_to_big_sum_70,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );verify_blake_word_evaluate(
            [((sha256_builtin_segment_start + (seq * qm31_const::<24, 0, 0, 0>())) + qm31_const::<23, 0, 0, 0>()), sha_256_round_output_limb_14_col110, sha_256_round_output_limb_15_col111],
low_7_ms_bits_col172,
high_14_ms_bits_col173,
high_5_ms_bits_col174,
output_7_id_col175,
self.range_check_7_2_5_lookup_elements,
self.memory_address_to_id_lookup_elements,
self.memory_id_to_big_lookup_elements,
ref range_check_7_2_5_sum_71,
ref memory_address_to_id_sum_72,
ref memory_id_to_big_sum_73,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );

        lookup_constraints(
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
            claimed_sum,
            column_size,
            ref interaction_trace_mask_values,
            range_check_7_2_5_sum_0,
memory_address_to_id_sum_1,
memory_id_to_big_sum_2,
range_check_7_2_5_sum_3,
memory_address_to_id_sum_4,
memory_id_to_big_sum_5,
range_check_7_2_5_sum_6,
memory_address_to_id_sum_7,
memory_id_to_big_sum_8,
range_check_7_2_5_sum_9,
memory_address_to_id_sum_10,
memory_id_to_big_sum_11,
range_check_7_2_5_sum_12,
memory_address_to_id_sum_13,
memory_id_to_big_sum_14,
range_check_7_2_5_sum_15,
memory_address_to_id_sum_16,
memory_id_to_big_sum_17,
range_check_7_2_5_sum_18,
memory_address_to_id_sum_19,
memory_id_to_big_sum_20,
range_check_7_2_5_sum_21,
memory_address_to_id_sum_22,
memory_id_to_big_sum_23,
range_check_7_2_5_sum_24,
memory_address_to_id_sum_25,
memory_id_to_big_sum_26,
range_check_7_2_5_sum_27,
memory_address_to_id_sum_28,
memory_id_to_big_sum_29,
range_check_7_2_5_sum_30,
memory_address_to_id_sum_31,
memory_id_to_big_sum_32,
range_check_7_2_5_sum_33,
memory_address_to_id_sum_34,
memory_id_to_big_sum_35,
range_check_7_2_5_sum_36,
memory_address_to_id_sum_37,
memory_id_to_big_sum_38,
range_check_7_2_5_sum_39,
memory_address_to_id_sum_40,
memory_id_to_big_sum_41,
range_check_7_2_5_sum_42,
memory_address_to_id_sum_43,
memory_id_to_big_sum_44,
range_check_7_2_5_sum_45,
memory_address_to_id_sum_46,
memory_id_to_big_sum_47,
sha_256_round_sum_48,
sha_256_round_sum_49,
range_check_7_2_5_sum_50,
memory_address_to_id_sum_51,
memory_id_to_big_sum_52,
range_check_7_2_5_sum_53,
memory_address_to_id_sum_54,
memory_id_to_big_sum_55,
range_check_7_2_5_sum_56,
memory_address_to_id_sum_57,
memory_id_to_big_sum_58,
range_check_7_2_5_sum_59,
memory_address_to_id_sum_60,
memory_id_to_big_sum_61,
range_check_7_2_5_sum_62,
memory_address_to_id_sum_63,
memory_id_to_big_sum_64,
range_check_7_2_5_sum_65,
memory_address_to_id_sum_66,
memory_id_to_big_sum_67,
range_check_7_2_5_sum_68,
memory_address_to_id_sum_69,
memory_id_to_big_sum_70,
range_check_7_2_5_sum_71,
memory_address_to_id_sum_72,
memory_id_to_big_sum_73
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
    range_check_7_2_5_sum_0: QM31,
memory_address_to_id_sum_1: QM31,
memory_id_to_big_sum_2: QM31,
range_check_7_2_5_sum_3: QM31,
memory_address_to_id_sum_4: QM31,
memory_id_to_big_sum_5: QM31,
range_check_7_2_5_sum_6: QM31,
memory_address_to_id_sum_7: QM31,
memory_id_to_big_sum_8: QM31,
range_check_7_2_5_sum_9: QM31,
memory_address_to_id_sum_10: QM31,
memory_id_to_big_sum_11: QM31,
range_check_7_2_5_sum_12: QM31,
memory_address_to_id_sum_13: QM31,
memory_id_to_big_sum_14: QM31,
range_check_7_2_5_sum_15: QM31,
memory_address_to_id_sum_16: QM31,
memory_id_to_big_sum_17: QM31,
range_check_7_2_5_sum_18: QM31,
memory_address_to_id_sum_19: QM31,
memory_id_to_big_sum_20: QM31,
range_check_7_2_5_sum_21: QM31,
memory_address_to_id_sum_22: QM31,
memory_id_to_big_sum_23: QM31,
range_check_7_2_5_sum_24: QM31,
memory_address_to_id_sum_25: QM31,
memory_id_to_big_sum_26: QM31,
range_check_7_2_5_sum_27: QM31,
memory_address_to_id_sum_28: QM31,
memory_id_to_big_sum_29: QM31,
range_check_7_2_5_sum_30: QM31,
memory_address_to_id_sum_31: QM31,
memory_id_to_big_sum_32: QM31,
range_check_7_2_5_sum_33: QM31,
memory_address_to_id_sum_34: QM31,
memory_id_to_big_sum_35: QM31,
range_check_7_2_5_sum_36: QM31,
memory_address_to_id_sum_37: QM31,
memory_id_to_big_sum_38: QM31,
range_check_7_2_5_sum_39: QM31,
memory_address_to_id_sum_40: QM31,
memory_id_to_big_sum_41: QM31,
range_check_7_2_5_sum_42: QM31,
memory_address_to_id_sum_43: QM31,
memory_id_to_big_sum_44: QM31,
range_check_7_2_5_sum_45: QM31,
memory_address_to_id_sum_46: QM31,
memory_id_to_big_sum_47: QM31,
sha_256_round_sum_48: QM31,
sha_256_round_sum_49: QM31,
range_check_7_2_5_sum_50: QM31,
memory_address_to_id_sum_51: QM31,
memory_id_to_big_sum_52: QM31,
range_check_7_2_5_sum_53: QM31,
memory_address_to_id_sum_54: QM31,
memory_id_to_big_sum_55: QM31,
range_check_7_2_5_sum_56: QM31,
memory_address_to_id_sum_57: QM31,
memory_id_to_big_sum_58: QM31,
range_check_7_2_5_sum_59: QM31,
memory_address_to_id_sum_60: QM31,
memory_id_to_big_sum_61: QM31,
range_check_7_2_5_sum_62: QM31,
memory_address_to_id_sum_63: QM31,
memory_id_to_big_sum_64: QM31,
range_check_7_2_5_sum_65: QM31,
memory_address_to_id_sum_66: QM31,
memory_id_to_big_sum_67: QM31,
range_check_7_2_5_sum_68: QM31,
memory_address_to_id_sum_69: QM31,
memory_id_to_big_sum_70: QM31,
range_check_7_2_5_sum_71: QM31,
memory_address_to_id_sum_72: QM31,
memory_id_to_big_sum_73: QM31
) {
    let [trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3, trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7, trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11, trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15, trace_2_col16, trace_2_col17, trace_2_col18, trace_2_col19, trace_2_col20, trace_2_col21, trace_2_col22, trace_2_col23, trace_2_col24, trace_2_col25, trace_2_col26, trace_2_col27, trace_2_col28, trace_2_col29, trace_2_col30, trace_2_col31, trace_2_col32, trace_2_col33, trace_2_col34, trace_2_col35, trace_2_col36, trace_2_col37, trace_2_col38, trace_2_col39, trace_2_col40, trace_2_col41, trace_2_col42, trace_2_col43, trace_2_col44, trace_2_col45, trace_2_col46, trace_2_col47, trace_2_col48, trace_2_col49, trace_2_col50, trace_2_col51, trace_2_col52, trace_2_col53, trace_2_col54, trace_2_col55, trace_2_col56, trace_2_col57, trace_2_col58, trace_2_col59, trace_2_col60, trace_2_col61, trace_2_col62, trace_2_col63, trace_2_col64, trace_2_col65, trace_2_col66, trace_2_col67, trace_2_col68, trace_2_col69, trace_2_col70, trace_2_col71, trace_2_col72, trace_2_col73, trace_2_col74, trace_2_col75, trace_2_col76, trace_2_col77, trace_2_col78, trace_2_col79, trace_2_col80, trace_2_col81, trace_2_col82, trace_2_col83, trace_2_col84, trace_2_col85, trace_2_col86, trace_2_col87, trace_2_col88, trace_2_col89, trace_2_col90, trace_2_col91, trace_2_col92, trace_2_col93, trace_2_col94, trace_2_col95, trace_2_col96, trace_2_col97, trace_2_col98, trace_2_col99, trace_2_col100, trace_2_col101, trace_2_col102, trace_2_col103, trace_2_col104, trace_2_col105, trace_2_col106, trace_2_col107, trace_2_col108, trace_2_col109, trace_2_col110, trace_2_col111, trace_2_col112, trace_2_col113, trace_2_col114, trace_2_col115, trace_2_col116, trace_2_col117, trace_2_col118, trace_2_col119, trace_2_col120, trace_2_col121, trace_2_col122, trace_2_col123, trace_2_col124, trace_2_col125, trace_2_col126, trace_2_col127, trace_2_col128, trace_2_col129, trace_2_col130, trace_2_col131, trace_2_col132, trace_2_col133, trace_2_col134, trace_2_col135, trace_2_col136, trace_2_col137, trace_2_col138, trace_2_col139, trace_2_col140, trace_2_col141, trace_2_col142, trace_2_col143, trace_2_col144, trace_2_col145, trace_2_col146, trace_2_col147]: [Span<QM31>; 148]
        = (*interaction_trace_mask_values.multi_pop_front().unwrap()).unbox();

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
let [trace_2_col104]: [QM31; 1] = (*trace_2_col104.try_into().unwrap()).unbox();
let [trace_2_col105]: [QM31; 1] = (*trace_2_col105.try_into().unwrap()).unbox();
let [trace_2_col106]: [QM31; 1] = (*trace_2_col106.try_into().unwrap()).unbox();
let [trace_2_col107]: [QM31; 1] = (*trace_2_col107.try_into().unwrap()).unbox();
let [trace_2_col108]: [QM31; 1] = (*trace_2_col108.try_into().unwrap()).unbox();
let [trace_2_col109]: [QM31; 1] = (*trace_2_col109.try_into().unwrap()).unbox();
let [trace_2_col110]: [QM31; 1] = (*trace_2_col110.try_into().unwrap()).unbox();
let [trace_2_col111]: [QM31; 1] = (*trace_2_col111.try_into().unwrap()).unbox();
let [trace_2_col112]: [QM31; 1] = (*trace_2_col112.try_into().unwrap()).unbox();
let [trace_2_col113]: [QM31; 1] = (*trace_2_col113.try_into().unwrap()).unbox();
let [trace_2_col114]: [QM31; 1] = (*trace_2_col114.try_into().unwrap()).unbox();
let [trace_2_col115]: [QM31; 1] = (*trace_2_col115.try_into().unwrap()).unbox();
let [trace_2_col116]: [QM31; 1] = (*trace_2_col116.try_into().unwrap()).unbox();
let [trace_2_col117]: [QM31; 1] = (*trace_2_col117.try_into().unwrap()).unbox();
let [trace_2_col118]: [QM31; 1] = (*trace_2_col118.try_into().unwrap()).unbox();
let [trace_2_col119]: [QM31; 1] = (*trace_2_col119.try_into().unwrap()).unbox();
let [trace_2_col120]: [QM31; 1] = (*trace_2_col120.try_into().unwrap()).unbox();
let [trace_2_col121]: [QM31; 1] = (*trace_2_col121.try_into().unwrap()).unbox();
let [trace_2_col122]: [QM31; 1] = (*trace_2_col122.try_into().unwrap()).unbox();
let [trace_2_col123]: [QM31; 1] = (*trace_2_col123.try_into().unwrap()).unbox();
let [trace_2_col124]: [QM31; 1] = (*trace_2_col124.try_into().unwrap()).unbox();
let [trace_2_col125]: [QM31; 1] = (*trace_2_col125.try_into().unwrap()).unbox();
let [trace_2_col126]: [QM31; 1] = (*trace_2_col126.try_into().unwrap()).unbox();
let [trace_2_col127]: [QM31; 1] = (*trace_2_col127.try_into().unwrap()).unbox();
let [trace_2_col128]: [QM31; 1] = (*trace_2_col128.try_into().unwrap()).unbox();
let [trace_2_col129]: [QM31; 1] = (*trace_2_col129.try_into().unwrap()).unbox();
let [trace_2_col130]: [QM31; 1] = (*trace_2_col130.try_into().unwrap()).unbox();
let [trace_2_col131]: [QM31; 1] = (*trace_2_col131.try_into().unwrap()).unbox();
let [trace_2_col132]: [QM31; 1] = (*trace_2_col132.try_into().unwrap()).unbox();
let [trace_2_col133]: [QM31; 1] = (*trace_2_col133.try_into().unwrap()).unbox();
let [trace_2_col134]: [QM31; 1] = (*trace_2_col134.try_into().unwrap()).unbox();
let [trace_2_col135]: [QM31; 1] = (*trace_2_col135.try_into().unwrap()).unbox();
let [trace_2_col136]: [QM31; 1] = (*trace_2_col136.try_into().unwrap()).unbox();
let [trace_2_col137]: [QM31; 1] = (*trace_2_col137.try_into().unwrap()).unbox();
let [trace_2_col138]: [QM31; 1] = (*trace_2_col138.try_into().unwrap()).unbox();
let [trace_2_col139]: [QM31; 1] = (*trace_2_col139.try_into().unwrap()).unbox();
let [trace_2_col140]: [QM31; 1] = (*trace_2_col140.try_into().unwrap()).unbox();
let [trace_2_col141]: [QM31; 1] = (*trace_2_col141.try_into().unwrap()).unbox();
let [trace_2_col142]: [QM31; 1] = (*trace_2_col142.try_into().unwrap()).unbox();
let [trace_2_col143]: [QM31; 1] = (*trace_2_col143.try_into().unwrap()).unbox();
let [trace_2_col144_neg1, trace_2_col144]: [QM31; 2] = (*trace_2_col144.try_into().unwrap()).unbox();
let [trace_2_col145_neg1, trace_2_col145]: [QM31; 2] = (*trace_2_col145.try_into().unwrap()).unbox();
let [trace_2_col146_neg1, trace_2_col146]: [QM31; 2] = (*trace_2_col146.try_into().unwrap()).unbox();
let [trace_2_col147_neg1, trace_2_col147]: [QM31; 2] = (*trace_2_col147.try_into().unwrap()).unbox();


    core::internal::revoke_ap_tracking();

    
let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3])
            ) * range_check_7_2_5_sum_0 * memory_address_to_id_sum_1
        ) - range_check_7_2_5_sum_0 - memory_address_to_id_sum_1
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7]) 
                - QM31Impl::from_partial_evals([trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3])
            ) * memory_id_to_big_sum_2 * range_check_7_2_5_sum_3
        ) - memory_id_to_big_sum_2 - range_check_7_2_5_sum_3
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11]) 
                - QM31Impl::from_partial_evals([trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7])
            ) * memory_address_to_id_sum_4 * memory_id_to_big_sum_5
        ) - memory_address_to_id_sum_4 - memory_id_to_big_sum_5
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15]) 
                - QM31Impl::from_partial_evals([trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11])
            ) * range_check_7_2_5_sum_6 * memory_address_to_id_sum_7
        ) - range_check_7_2_5_sum_6 - memory_address_to_id_sum_7
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col16, trace_2_col17, trace_2_col18, trace_2_col19]) 
                - QM31Impl::from_partial_evals([trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15])
            ) * memory_id_to_big_sum_8 * range_check_7_2_5_sum_9
        ) - memory_id_to_big_sum_8 - range_check_7_2_5_sum_9
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col20, trace_2_col21, trace_2_col22, trace_2_col23]) 
                - QM31Impl::from_partial_evals([trace_2_col16, trace_2_col17, trace_2_col18, trace_2_col19])
            ) * memory_address_to_id_sum_10 * memory_id_to_big_sum_11
        ) - memory_address_to_id_sum_10 - memory_id_to_big_sum_11
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col24, trace_2_col25, trace_2_col26, trace_2_col27]) 
                - QM31Impl::from_partial_evals([trace_2_col20, trace_2_col21, trace_2_col22, trace_2_col23])
            ) * range_check_7_2_5_sum_12 * memory_address_to_id_sum_13
        ) - range_check_7_2_5_sum_12 - memory_address_to_id_sum_13
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col28, trace_2_col29, trace_2_col30, trace_2_col31]) 
                - QM31Impl::from_partial_evals([trace_2_col24, trace_2_col25, trace_2_col26, trace_2_col27])
            ) * memory_id_to_big_sum_14 * range_check_7_2_5_sum_15
        ) - memory_id_to_big_sum_14 - range_check_7_2_5_sum_15
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col32, trace_2_col33, trace_2_col34, trace_2_col35]) 
                - QM31Impl::from_partial_evals([trace_2_col28, trace_2_col29, trace_2_col30, trace_2_col31])
            ) * memory_address_to_id_sum_16 * memory_id_to_big_sum_17
        ) - memory_address_to_id_sum_16 - memory_id_to_big_sum_17
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col36, trace_2_col37, trace_2_col38, trace_2_col39]) 
                - QM31Impl::from_partial_evals([trace_2_col32, trace_2_col33, trace_2_col34, trace_2_col35])
            ) * range_check_7_2_5_sum_18 * memory_address_to_id_sum_19
        ) - range_check_7_2_5_sum_18 - memory_address_to_id_sum_19
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col40, trace_2_col41, trace_2_col42, trace_2_col43]) 
                - QM31Impl::from_partial_evals([trace_2_col36, trace_2_col37, trace_2_col38, trace_2_col39])
            ) * memory_id_to_big_sum_20 * range_check_7_2_5_sum_21
        ) - memory_id_to_big_sum_20 - range_check_7_2_5_sum_21
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col44, trace_2_col45, trace_2_col46, trace_2_col47]) 
                - QM31Impl::from_partial_evals([trace_2_col40, trace_2_col41, trace_2_col42, trace_2_col43])
            ) * memory_address_to_id_sum_22 * memory_id_to_big_sum_23
        ) - memory_address_to_id_sum_22 - memory_id_to_big_sum_23
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col48, trace_2_col49, trace_2_col50, trace_2_col51]) 
                - QM31Impl::from_partial_evals([trace_2_col44, trace_2_col45, trace_2_col46, trace_2_col47])
            ) * range_check_7_2_5_sum_24 * memory_address_to_id_sum_25
        ) - range_check_7_2_5_sum_24 - memory_address_to_id_sum_25
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col52, trace_2_col53, trace_2_col54, trace_2_col55]) 
                - QM31Impl::from_partial_evals([trace_2_col48, trace_2_col49, trace_2_col50, trace_2_col51])
            ) * memory_id_to_big_sum_26 * range_check_7_2_5_sum_27
        ) - memory_id_to_big_sum_26 - range_check_7_2_5_sum_27
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col56, trace_2_col57, trace_2_col58, trace_2_col59]) 
                - QM31Impl::from_partial_evals([trace_2_col52, trace_2_col53, trace_2_col54, trace_2_col55])
            ) * memory_address_to_id_sum_28 * memory_id_to_big_sum_29
        ) - memory_address_to_id_sum_28 - memory_id_to_big_sum_29
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col60, trace_2_col61, trace_2_col62, trace_2_col63]) 
                - QM31Impl::from_partial_evals([trace_2_col56, trace_2_col57, trace_2_col58, trace_2_col59])
            ) * range_check_7_2_5_sum_30 * memory_address_to_id_sum_31
        ) - range_check_7_2_5_sum_30 - memory_address_to_id_sum_31
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col64, trace_2_col65, trace_2_col66, trace_2_col67]) 
                - QM31Impl::from_partial_evals([trace_2_col60, trace_2_col61, trace_2_col62, trace_2_col63])
            ) * memory_id_to_big_sum_32 * range_check_7_2_5_sum_33
        ) - memory_id_to_big_sum_32 - range_check_7_2_5_sum_33
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col68, trace_2_col69, trace_2_col70, trace_2_col71]) 
                - QM31Impl::from_partial_evals([trace_2_col64, trace_2_col65, trace_2_col66, trace_2_col67])
            ) * memory_address_to_id_sum_34 * memory_id_to_big_sum_35
        ) - memory_address_to_id_sum_34 - memory_id_to_big_sum_35
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col72, trace_2_col73, trace_2_col74, trace_2_col75]) 
                - QM31Impl::from_partial_evals([trace_2_col68, trace_2_col69, trace_2_col70, trace_2_col71])
            ) * range_check_7_2_5_sum_36 * memory_address_to_id_sum_37
        ) - range_check_7_2_5_sum_36 - memory_address_to_id_sum_37
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col76, trace_2_col77, trace_2_col78, trace_2_col79]) 
                - QM31Impl::from_partial_evals([trace_2_col72, trace_2_col73, trace_2_col74, trace_2_col75])
            ) * memory_id_to_big_sum_38 * range_check_7_2_5_sum_39
        ) - memory_id_to_big_sum_38 - range_check_7_2_5_sum_39
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col80, trace_2_col81, trace_2_col82, trace_2_col83]) 
                - QM31Impl::from_partial_evals([trace_2_col76, trace_2_col77, trace_2_col78, trace_2_col79])
            ) * memory_address_to_id_sum_40 * memory_id_to_big_sum_41
        ) - memory_address_to_id_sum_40 - memory_id_to_big_sum_41
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col84, trace_2_col85, trace_2_col86, trace_2_col87]) 
                - QM31Impl::from_partial_evals([trace_2_col80, trace_2_col81, trace_2_col82, trace_2_col83])
            ) * range_check_7_2_5_sum_42 * memory_address_to_id_sum_43
        ) - range_check_7_2_5_sum_42 - memory_address_to_id_sum_43
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col88, trace_2_col89, trace_2_col90, trace_2_col91]) 
                - QM31Impl::from_partial_evals([trace_2_col84, trace_2_col85, trace_2_col86, trace_2_col87])
            ) * memory_id_to_big_sum_44 * range_check_7_2_5_sum_45
        ) - memory_id_to_big_sum_44 - range_check_7_2_5_sum_45
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col92, trace_2_col93, trace_2_col94, trace_2_col95]) 
                - QM31Impl::from_partial_evals([trace_2_col88, trace_2_col89, trace_2_col90, trace_2_col91])
            ) * memory_address_to_id_sum_46 * memory_id_to_big_sum_47
        ) - memory_address_to_id_sum_46 - memory_id_to_big_sum_47
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col96, trace_2_col97, trace_2_col98, trace_2_col99]) 
                - QM31Impl::from_partial_evals([trace_2_col92, trace_2_col93, trace_2_col94, trace_2_col95])
            ) * sha_256_round_sum_48 * sha_256_round_sum_49
        ) - sha_256_round_sum_48 + sha_256_round_sum_49
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col100, trace_2_col101, trace_2_col102, trace_2_col103]) 
                - QM31Impl::from_partial_evals([trace_2_col96, trace_2_col97, trace_2_col98, trace_2_col99])
            ) * range_check_7_2_5_sum_50 * memory_address_to_id_sum_51
        ) - range_check_7_2_5_sum_50 - memory_address_to_id_sum_51
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col104, trace_2_col105, trace_2_col106, trace_2_col107]) 
                - QM31Impl::from_partial_evals([trace_2_col100, trace_2_col101, trace_2_col102, trace_2_col103])
            ) * memory_id_to_big_sum_52 * range_check_7_2_5_sum_53
        ) - memory_id_to_big_sum_52 - range_check_7_2_5_sum_53
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col108, trace_2_col109, trace_2_col110, trace_2_col111]) 
                - QM31Impl::from_partial_evals([trace_2_col104, trace_2_col105, trace_2_col106, trace_2_col107])
            ) * memory_address_to_id_sum_54 * memory_id_to_big_sum_55
        ) - memory_address_to_id_sum_54 - memory_id_to_big_sum_55
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col112, trace_2_col113, trace_2_col114, trace_2_col115]) 
                - QM31Impl::from_partial_evals([trace_2_col108, trace_2_col109, trace_2_col110, trace_2_col111])
            ) * range_check_7_2_5_sum_56 * memory_address_to_id_sum_57
        ) - range_check_7_2_5_sum_56 - memory_address_to_id_sum_57
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col116, trace_2_col117, trace_2_col118, trace_2_col119]) 
                - QM31Impl::from_partial_evals([trace_2_col112, trace_2_col113, trace_2_col114, trace_2_col115])
            ) * memory_id_to_big_sum_58 * range_check_7_2_5_sum_59
        ) - memory_id_to_big_sum_58 - range_check_7_2_5_sum_59
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col120, trace_2_col121, trace_2_col122, trace_2_col123]) 
                - QM31Impl::from_partial_evals([trace_2_col116, trace_2_col117, trace_2_col118, trace_2_col119])
            ) * memory_address_to_id_sum_60 * memory_id_to_big_sum_61
        ) - memory_address_to_id_sum_60 - memory_id_to_big_sum_61
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col124, trace_2_col125, trace_2_col126, trace_2_col127]) 
                - QM31Impl::from_partial_evals([trace_2_col120, trace_2_col121, trace_2_col122, trace_2_col123])
            ) * range_check_7_2_5_sum_62 * memory_address_to_id_sum_63
        ) - range_check_7_2_5_sum_62 - memory_address_to_id_sum_63
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col128, trace_2_col129, trace_2_col130, trace_2_col131]) 
                - QM31Impl::from_partial_evals([trace_2_col124, trace_2_col125, trace_2_col126, trace_2_col127])
            ) * memory_id_to_big_sum_64 * range_check_7_2_5_sum_65
        ) - memory_id_to_big_sum_64 - range_check_7_2_5_sum_65
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col132, trace_2_col133, trace_2_col134, trace_2_col135]) 
                - QM31Impl::from_partial_evals([trace_2_col128, trace_2_col129, trace_2_col130, trace_2_col131])
            ) * memory_address_to_id_sum_66 * memory_id_to_big_sum_67
        ) - memory_address_to_id_sum_66 - memory_id_to_big_sum_67
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col136, trace_2_col137, trace_2_col138, trace_2_col139]) 
                - QM31Impl::from_partial_evals([trace_2_col132, trace_2_col133, trace_2_col134, trace_2_col135])
            ) * range_check_7_2_5_sum_68 * memory_address_to_id_sum_69
        ) - range_check_7_2_5_sum_68 - memory_address_to_id_sum_69
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col140, trace_2_col141, trace_2_col142, trace_2_col143]) 
                - QM31Impl::from_partial_evals([trace_2_col136, trace_2_col137, trace_2_col138, trace_2_col139])
            ) * memory_id_to_big_sum_70 * range_check_7_2_5_sum_71
        ) - memory_id_to_big_sum_70 - range_check_7_2_5_sum_71
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col144, trace_2_col145, trace_2_col146, trace_2_col147]) 
                - QM31Impl::from_partial_evals([trace_2_col140, trace_2_col141, trace_2_col142, trace_2_col143]) 
                - QM31Impl::from_partial_evals([trace_2_col144_neg1, trace_2_col145_neg1, trace_2_col146_neg1, trace_2_col147_neg1])
                + (claimed_sum * (column_size.inverse().into()))
            ) * memory_address_to_id_sum_72 * memory_id_to_big_sum_73
        ) - memory_address_to_id_sum_72 - memory_id_to_big_sum_73
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

}