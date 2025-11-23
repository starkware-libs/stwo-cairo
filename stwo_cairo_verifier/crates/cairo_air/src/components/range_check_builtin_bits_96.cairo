// This file was created by the AIR team.

use crate::components::subroutines::read_positive_num_bits_96::read_positive_num_bits_96_evaluate;
use crate::prelude::*;

pub const N_TRACE_COLUMNS: usize = 12;
pub const RELATION_USES_PER_ROW: [(felt252, u32); 3] = [
    ('MemoryAddressToId', 1), ('RangeCheck_6', 1), ('MemoryIdToBig', 1),
];

#[derive(Drop, Serde, Copy)]
pub struct Claim {
    pub log_size: u32,
    pub range_check96_builtin_segment_start: u32,
}

pub impl ClaimImpl of ClaimTrait<Claim> {
    fn log_sizes(self: @Claim) -> TreeArray<Span<u32>> {
        let log_size = *(self.log_size);
        let preprocessed_log_sizes = array![log_size].span();
        let trace_log_sizes = [log_size; N_TRACE_COLUMNS].span();
        let interaction_log_sizes = [log_size; 8].span();
        array![preprocessed_log_sizes, trace_log_sizes, interaction_log_sizes]
    }

    fn mix_into(self: @Claim, ref channel: Channel) {
        channel.mix_u64((*(self.log_size)).into());
        channel.mix_u64((*self.range_check96_builtin_segment_start).into());
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
    pub range_check_6_lookup_elements: crate::RangeCheck_6Elements,
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
            range_check_6_lookup_elements: interaction_elements.range_checks.rc_6.clone(),
            memory_id_to_big_lookup_elements: interaction_elements.memory_id_to_value.clone(),
        }
    }
}

pub impl CairoComponentImpl of CairoComponent<Component> {
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
        let range_check96_builtin_segment_start: QM31 = (TryInto::<
            u32, M31,
        >::try_into((*(self.claim.range_check96_builtin_segment_start)))
            .unwrap())
            .into();
        let mut memory_address_to_id_sum_0: QM31 = Zero::zero();
        let mut range_check_6_sum_1: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_2: QM31 = Zero::zero();
        let seq = preprocessed_mask_values
            .get_and_mark_used(seq_column_idx(*(self.claim.log_size)));

        let [
            value_id_col0,
            value_limb_0_col1,
            value_limb_1_col2,
            value_limb_2_col3,
            value_limb_3_col4,
            value_limb_4_col5,
            value_limb_5_col6,
            value_limb_6_col7,
            value_limb_7_col8,
            value_limb_8_col9,
            value_limb_9_col10,
            value_limb_10_col11,
        ]: [Span<QM31>; 12] =
            (*trace_mask_values
            .multi_pop_front()
            .unwrap())
            .unbox();
        let [value_id_col0]: [QM31; 1] = (*value_id_col0.try_into().unwrap()).unbox();
        let [value_limb_0_col1]: [QM31; 1] = (*value_limb_0_col1.try_into().unwrap()).unbox();
        let [value_limb_1_col2]: [QM31; 1] = (*value_limb_1_col2.try_into().unwrap()).unbox();
        let [value_limb_2_col3]: [QM31; 1] = (*value_limb_2_col3.try_into().unwrap()).unbox();
        let [value_limb_3_col4]: [QM31; 1] = (*value_limb_3_col4.try_into().unwrap()).unbox();
        let [value_limb_4_col5]: [QM31; 1] = (*value_limb_4_col5.try_into().unwrap()).unbox();
        let [value_limb_5_col6]: [QM31; 1] = (*value_limb_5_col6.try_into().unwrap()).unbox();
        let [value_limb_6_col7]: [QM31; 1] = (*value_limb_6_col7.try_into().unwrap()).unbox();
        let [value_limb_7_col8]: [QM31; 1] = (*value_limb_7_col8.try_into().unwrap()).unbox();
        let [value_limb_8_col9]: [QM31; 1] = (*value_limb_8_col9.try_into().unwrap()).unbox();
        let [value_limb_9_col10]: [QM31; 1] = (*value_limb_9_col10.try_into().unwrap()).unbox();
        let [value_limb_10_col11]: [QM31; 1] = (*value_limb_10_col11.try_into().unwrap()).unbox();

        core::internal::revoke_ap_tracking();

        read_positive_num_bits_96_evaluate(
            (range_check96_builtin_segment_start + seq),
            value_id_col0,
            value_limb_0_col1,
            value_limb_1_col2,
            value_limb_2_col3,
            value_limb_3_col4,
            value_limb_4_col5,
            value_limb_5_col6,
            value_limb_6_col7,
            value_limb_7_col8,
            value_limb_8_col9,
            value_limb_9_col10,
            value_limb_10_col11,
            self.memory_address_to_id_lookup_elements,
            self.range_check_6_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref memory_address_to_id_sum_0,
            ref range_check_6_sum_1,
            ref memory_id_to_big_sum_2,
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
            memory_address_to_id_sum_0,
            range_check_6_sum_1,
            memory_id_to_big_sum_2,
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
    range_check_6_sum_1: QM31,
    memory_id_to_big_sum_2: QM31,
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
    ]: [Span<QM31>; 8] =
        (*interaction_trace_mask_values
        .multi_pop_front()
        .unwrap())
        .unbox();

    let [trace_2_col0]: [QM31; 1] = (*trace_2_col0.try_into().unwrap()).unbox();
    let [trace_2_col1]: [QM31; 1] = (*trace_2_col1.try_into().unwrap()).unbox();
    let [trace_2_col2]: [QM31; 1] = (*trace_2_col2.try_into().unwrap()).unbox();
    let [trace_2_col3]: [QM31; 1] = (*trace_2_col3.try_into().unwrap()).unbox();
    let [trace_2_col4_neg1, trace_2_col4]: [QM31; 2] = (*trace_2_col4.try_into().unwrap()).unbox();
    let [trace_2_col5_neg1, trace_2_col5]: [QM31; 2] = (*trace_2_col5.try_into().unwrap()).unbox();
    let [trace_2_col6_neg1, trace_2_col6]: [QM31; 2] = (*trace_2_col6.try_into().unwrap()).unbox();
    let [trace_2_col7_neg1, trace_2_col7]: [QM31; 2] = (*trace_2_col7.try_into().unwrap()).unbox();

    core::internal::revoke_ap_tracking();

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3],
    ))
        * memory_address_to_id_sum_0
        * range_check_6_sum_1)
        - memory_address_to_id_sum_0
        - range_check_6_sum_1)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7],
    )
        - QM31Impl::from_partial_evals([trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3])
        - QM31Impl::from_partial_evals(
            [trace_2_col4_neg1, trace_2_col5_neg1, trace_2_col6_neg1, trace_2_col7_neg1],
        )
        + (claimed_sum * (column_size.inverse().into())))
        * memory_id_to_big_sum_2)
        - qm31_const::<1, 0, 0, 0>())
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
}
#[cfg(and(test, feature: "qm31_opcode"))]
mod tests {
    use core::array::ArrayImpl;
    use core::num::traits::Zero;
    #[allow(unused_imports)]
    use stwo_cairo_air::preprocessed_columns::{NUM_PREPROCESSED_COLUMNS, seq_column_idx};
    #[allow(unused_imports)]
    use stwo_constraint_framework::{
        LookupElements, PreprocessedMaskValues, PreprocessedMaskValuesTrait,
    };
    use stwo_verifier_core::circle::CirclePoint;
    use stwo_verifier_core::fields::qm31::{QM31, QM31Impl, QM31Trait, qm31_const};
    use crate::cairo_component::*;
    use crate::components::sample_evaluations::*;
    #[allow(unused_imports)]
    use crate::test_utils::{make_interaction_trace, make_lookup_elements, preprocessed_mask_add};
    use crate::utils::*;
    use super::{Claim, Component, InteractionClaim};

    #[test]
    fn test_evaluation_result() {
        let component = Component {
            claim: Claim { log_size: 15, range_check96_builtin_segment_start: 939053492 },
            interaction_claim: InteractionClaim {
                claimed_sum: qm31_const::<1398335417, 314974026, 1722107152, 821933968>(),
            },
            memory_address_to_id_lookup_elements: make_lookup_elements(
                qm31_const::<1842771211, 1960835386, 1582137647, 1333140033>(),
                qm31_const::<1360491305, 950648792, 556642685, 2096522554>(),
            ),
            memory_id_to_big_lookup_elements: make_lookup_elements(
                qm31_const::<844624398, 1166453613, 1247584074, 330174372>(),
                qm31_const::<1844105245, 1400976933, 1126903288, 1155460729>(),
            ),
            range_check_6_lookup_elements: make_lookup_elements(
                qm31_const::<305339001, 974590906, 1066031859, 439859987>(),
                qm31_const::<992075190, 1422459785, 377626248, 1033323458>(),
            ),
        };
        let mut sum: QM31 = Zero::zero();
        let point = CirclePoint {
            x: qm31_const::<461666434, 38651694, 1083586041, 510305943>(),
            y: qm31_const::<817798294, 862569777, 2091320744, 1178484122>(),
        };

        let mut preprocessed_trace = PreprocessedMaskValues { values: Default::default() };
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            seq_column_idx(component.claim.log_size),
            qm31_const::<735272696, 1215403647, 795393303, 879304430>(),
        );

        let mut trace_columns = [
            [qm31_const::<1659099300, 905558730, 651199673, 1375009625>()].span(),
            [qm31_const::<1591990121, 771341002, 584090809, 1375009625>()].span(),
            [qm31_const::<1793317658, 1173994186, 785417401, 1375009625>()].span(),
            [qm31_const::<1726208479, 1039776458, 718308537, 1375009625>()].span(),
            [qm31_const::<1390662584, 368687818, 382764217, 1375009625>()].span(),
            [qm31_const::<1323553405, 234470090, 315655353, 1375009625>()].span(),
            [qm31_const::<1524880942, 637123274, 516981945, 1375009625>()].span(),
            [qm31_const::<1457771763, 502905546, 449873081, 1375009625>()].span(),
            [qm31_const::<48489085, 1979300555, 1188070585, 1375009625>()].span(),
            [qm31_const::<2128863553, 1845082826, 1120961721, 1375009625>()].span(),
            [qm31_const::<1852335767, 645078115, 2059236183, 343880121>()].span(),
            [qm31_const::<1919444946, 779295843, 2126345047, 343880121>()].span(),
        ]
            .span();
        let interaction_values = array![
            qm31_const::<1005168032, 79980996, 1847888101, 1941984119>(),
            qm31_const::<1072277211, 214198724, 1914996965, 1941984119>(),
        ];
        let mut interaction_columns = make_interaction_trace(
            interaction_values, qm31_const::<1115374022, 1127856551, 489657863, 643630026>(),
        );
        component
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_trace,
                ref trace_columns,
                ref interaction_columns,
                qm31_const::<474642921, 876336632, 1911695779, 974600512>(),
                point,
            );
        preprocessed_trace.validate_usage();
        assert_eq!(sum, QM31Trait::from_fixed_array(RANGE_CHECK_BUILTIN_BITS_96_SAMPLE_EVAL_RESULT))
    }
}
