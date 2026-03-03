// This file was created by the AIR team.

use crate::components::subroutines::mem_verify::mem_verify_evaluate;
use crate::prelude::*;

pub const N_TRACE_COLUMNS: usize = 1;
pub const RELATION_USES_PER_ROW: [(felt252, u32); 2] = [
    ('MemoryAddressToId', 1), ('MemoryIdToBig', 1),
];

#[derive(Drop, Serde, Copy)]
pub struct Claim {
    pub log_size: u32,
}

pub impl ClaimImpl of ClaimTrait<Claim> {
    fn log_sizes(self: @Claim) -> TreeArray<Span<u32>> {
        let log_size = *(self.log_size);
        let preprocessed_log_sizes = array![log_size].span();
        let trace_log_sizes = [log_size; N_TRACE_COLUMNS].span();
        let interaction_log_sizes = [log_size; 4].span();
        array![preprocessed_log_sizes, trace_log_sizes, interaction_log_sizes]
    }

    fn mix_into(self: @Claim, ref channel: Channel) {
        channel.mix_u64((*(self.log_size)).into());
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
    pub common_lookup_elements: CommonLookupElements,
}

pub impl NewComponentImpl of NewComponent<Component> {
    type Claim = Claim;
    type InteractionClaim = InteractionClaim;

    fn new(
        claim: @Claim,
        interaction_claim: @InteractionClaim,
        common_lookup_elements: @CommonLookupElements,
    ) -> Component {
        Component {
            claim: *claim,
            interaction_claim: *interaction_claim,
            common_lookup_elements: common_lookup_elements.clone(),
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
    ) {
        let log_size = *(self.claim.log_size);
        let claimed_sum = *self.interaction_claim.claimed_sum;
        let column_size = m31(pow2(log_size));
        let mut memory_address_to_id_sum_0: QM31 = Zero::zero();
        let mut numerator_0: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_1: QM31 = Zero::zero();
        let mut numerator_1: QM31 = Zero::zero();
        let seq_26 = preprocessed_mask_values.get_and_mark_used(SEQ_26_IDX);
        let curr_program_0 = preprocessed_mask_values.get_and_mark_used(CURR_PROGRAM_0_IDX);
        let curr_program_1 = preprocessed_mask_values.get_and_mark_used(CURR_PROGRAM_1_IDX);
        let curr_program_2 = preprocessed_mask_values.get_and_mark_used(CURR_PROGRAM_2_IDX);
        let curr_program_3 = preprocessed_mask_values.get_and_mark_used(CURR_PROGRAM_3_IDX);
        let curr_program_4 = preprocessed_mask_values.get_and_mark_used(CURR_PROGRAM_4_IDX);
        let curr_program_5 = preprocessed_mask_values.get_and_mark_used(CURR_PROGRAM_5_IDX);
        let curr_program_6 = preprocessed_mask_values.get_and_mark_used(CURR_PROGRAM_6_IDX);
        let curr_program_7 = preprocessed_mask_values.get_and_mark_used(CURR_PROGRAM_7_IDX);
        let curr_program_8 = preprocessed_mask_values.get_and_mark_used(CURR_PROGRAM_8_IDX);
        let curr_program_9 = preprocessed_mask_values.get_and_mark_used(CURR_PROGRAM_9_IDX);
        let curr_program_10 = preprocessed_mask_values.get_and_mark_used(CURR_PROGRAM_10_IDX);
        let curr_program_11 = preprocessed_mask_values.get_and_mark_used(CURR_PROGRAM_11_IDX);
        let curr_program_12 = preprocessed_mask_values.get_and_mark_used(CURR_PROGRAM_12_IDX);
        let curr_program_13 = preprocessed_mask_values.get_and_mark_used(CURR_PROGRAM_13_IDX);
        let curr_program_14 = preprocessed_mask_values.get_and_mark_used(CURR_PROGRAM_14_IDX);
        let curr_program_15 = preprocessed_mask_values.get_and_mark_used(CURR_PROGRAM_15_IDX);
        let curr_program_16 = preprocessed_mask_values.get_and_mark_used(CURR_PROGRAM_16_IDX);
        let curr_program_17 = preprocessed_mask_values.get_and_mark_used(CURR_PROGRAM_17_IDX);
        let curr_program_18 = preprocessed_mask_values.get_and_mark_used(CURR_PROGRAM_18_IDX);
        let curr_program_19 = preprocessed_mask_values.get_and_mark_used(CURR_PROGRAM_19_IDX);
        let curr_program_20 = preprocessed_mask_values.get_and_mark_used(CURR_PROGRAM_20_IDX);
        let curr_program_21 = preprocessed_mask_values.get_and_mark_used(CURR_PROGRAM_21_IDX);
        let curr_program_22 = preprocessed_mask_values.get_and_mark_used(CURR_PROGRAM_22_IDX);
        let curr_program_23 = preprocessed_mask_values.get_and_mark_used(CURR_PROGRAM_23_IDX);
        let curr_program_24 = preprocessed_mask_values.get_and_mark_used(CURR_PROGRAM_24_IDX);
        let curr_program_25 = preprocessed_mask_values.get_and_mark_used(CURR_PROGRAM_25_IDX);
        let curr_program_26 = preprocessed_mask_values.get_and_mark_used(CURR_PROGRAM_26_IDX);
        let curr_program_27 = preprocessed_mask_values.get_and_mark_used(CURR_PROGRAM_27_IDX);

        let [address_id_col0]: [Span<QM31>; 1] = (*trace_mask_values.multi_pop_front().unwrap())
            .unbox();
        let [address_id_col0]: [QM31; 1] = (*address_id_col0.try_into().unwrap()).unbox();

        core::internal::revoke_ap_tracking();

        mem_verify_evaluate(
            [
                seq_26, curr_program_0, curr_program_1,
                curr_program_2, curr_program_3, curr_program_4, curr_program_5, curr_program_6,
                curr_program_7, curr_program_8, curr_program_9, curr_program_10, curr_program_11,
                curr_program_12, curr_program_13, curr_program_14, curr_program_15, curr_program_16,
                curr_program_17, curr_program_18, curr_program_19, curr_program_20, curr_program_21,
                curr_program_22, curr_program_23, curr_program_24, curr_program_25, curr_program_26,
                curr_program_27,
            ],
            address_id_col0,
            self.common_lookup_elements,
            ref memory_address_to_id_sum_0,
            ref numerator_0,
            ref memory_id_to_big_sum_1,
            ref numerator_1,
            ref sum,
            random_coeff,
        );

        lookup_constraints(
            ref sum,
            random_coeff,
            claimed_sum,
            numerator_0,
            numerator_1,
            column_size,
            ref interaction_trace_mask_values,
            memory_address_to_id_sum_0,
            memory_id_to_big_sum_1,
        );
    }
}


fn lookup_constraints(
    ref sum: QM31,
    random_coeff: QM31,
    claimed_sum: QM31,
    numerator_0: QM31,
    numerator_1: QM31,
    column_size: M31,
    ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
    memory_address_to_id_sum_0: QM31,
    memory_id_to_big_sum_1: QM31,
) {
    let [trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3]: [Span<QM31>; 4] =
        (*interaction_trace_mask_values
        .multi_pop_front()
        .unwrap())
        .unbox();

    let [trace_2_col0_neg1, trace_2_col0]: [QM31; 2] = (*trace_2_col0.try_into().unwrap()).unbox();
    let [trace_2_col1_neg1, trace_2_col1]: [QM31; 2] = (*trace_2_col1.try_into().unwrap()).unbox();
    let [trace_2_col2_neg1, trace_2_col2]: [QM31; 2] = (*trace_2_col2.try_into().unwrap()).unbox();
    let [trace_2_col3_neg1, trace_2_col3]: [QM31; 2] = (*trace_2_col3.try_into().unwrap()).unbox();

    core::internal::revoke_ap_tracking();

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col0_neg1, trace_2_col1_neg1, trace_2_col2_neg1, trace_2_col3_neg1],
        )
        + (claimed_sum * (column_size.inverse().into())))
        * memory_address_to_id_sum_0
        * memory_id_to_big_sum_1)
        - (memory_address_to_id_sum_0 * numerator_1)
        - (memory_id_to_big_sum_1 * numerator_0));
    sum = sum * random_coeff + constraint_quotient;
}
#[cfg(and(test, feature: "qm31_opcode"))]
mod tests {
    use core::array::ArrayImpl;
    use core::num::traits::Zero;
    #[allow(unused_imports)]
    use stwo_cairo_air::preprocessed_columns::*;
    #[allow(unused_imports)]
    use stwo_constraint_framework::{
        LookupElementsTrait, PreprocessedMaskValues, PreprocessedMaskValuesTrait,
    };
    use stwo_verifier_core::fields::qm31::{QM31, QM31Impl, QM31Trait, qm31_const};
    use crate::cairo_component::*;
    use crate::components::sample_evaluations::*;
    #[allow(unused_imports)]
    use crate::test_utils::{make_interaction_trace, preprocessed_mask_add};
    use crate::utils::*;
    use super::{Claim, Component, InteractionClaim};

    #[test]
    fn test_evaluation_result() {
        let component = Component {
            claim: Claim { log_size: 15 },
            interaction_claim: InteractionClaim {
                claimed_sum: qm31_const::<1398335417, 314974026, 1722107152, 821933968>(),
            },
            common_lookup_elements: LookupElementsTrait::from_z_alpha(
                qm31_const::<445623802, 202571636, 1360224996, 131355117>(),
                qm31_const::<476823935, 939223384, 62486082, 122423602>(),
            ),
        };
        let mut sum: QM31 = Zero::zero();

        let mut preprocessed_trace = PreprocessedMaskValues { values: Default::default() };
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            seq_column_idx(component.claim.log_size),
            qm31_const::<735272696, 1215403647, 795393303, 879304430>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            CURR_PROGRAM_0_IDX,
            qm31_const::<1472218168, 262064501, 2030931257, 1197883171>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            CURR_PROGRAM_1_IDX,
            qm31_const::<1405108989, 127846773, 1963822393, 1197883171>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            CURR_PROGRAM_2_IDX,
            qm31_const::<1337999810, 2141112692, 1896713528, 1197883171>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            CURR_PROGRAM_3_IDX,
            qm31_const::<1270890631, 2006894964, 1829604664, 1197883171>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            CURR_PROGRAM_4_IDX,
            qm31_const::<1740654884, 798935413, 151883066, 1197883172>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            CURR_PROGRAM_5_IDX,
            qm31_const::<1673545705, 664717685, 84774202, 1197883172>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            CURR_PROGRAM_6_IDX,
            qm31_const::<1606436526, 530499957, 17665338, 1197883172>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            CURR_PROGRAM_7_IDX,
            qm31_const::<1539327347, 396282229, 2098040121, 1197883171>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            CURR_PROGRAM_8_IDX,
            qm31_const::<935344736, 1335806324, 1494060344, 1197883171>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            CURR_PROGRAM_9_IDX,
            qm31_const::<868235557, 1201588596, 1426951480, 1197883171>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            CURR_PROGRAM_10_IDX,
            qm31_const::<2036385629, 1808577500, 1528272129, 245233003>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            CURR_PROGRAM_11_IDX,
            qm31_const::<2103494808, 1942795228, 1595380993, 245233003>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            CURR_PROGRAM_12_IDX,
            qm31_const::<1902167271, 1540142044, 1394054401, 245233003>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            CURR_PROGRAM_13_IDX,
            qm31_const::<1969276450, 1674359772, 1461163265, 245233003>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            CURR_PROGRAM_14_IDX,
            qm31_const::<1767948913, 1271706588, 1259836673, 245233003>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            CURR_PROGRAM_15_IDX,
            qm31_const::<1835058092, 1405924316, 1326945537, 245233003>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            CURR_PROGRAM_16_IDX,
            qm31_const::<1633730555, 1003271132, 1125618945, 245233003>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            CURR_PROGRAM_17_IDX,
            qm31_const::<1700839734, 1137488860, 1192727809, 245233003>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            CURR_PROGRAM_18_IDX,
            qm31_const::<1499512197, 734835676, 991401217, 245233003>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            CURR_PROGRAM_19_IDX,
            qm31_const::<1566621376, 869053404, 1058510081, 245233003>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            CURR_PROGRAM_20_IDX,
            qm31_const::<559881296, 1003271071, 51877060, 245232983>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            CURR_PROGRAM_21_IDX,
            qm31_const::<492772117, 869053343, 2132251843, 245232982>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            CURR_PROGRAM_22_IDX,
            qm31_const::<425662938, 734835615, 2065142979, 245232982>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            CURR_PROGRAM_23_IDX,
            qm31_const::<358553759, 600617887, 1998034115, 245232982>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            CURR_PROGRAM_24_IDX,
            qm31_const::<828318012, 1540141983, 320312516, 245232983>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            CURR_PROGRAM_25_IDX,
            qm31_const::<761208833, 1405924255, 253203652, 245232983>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            CURR_PROGRAM_26_IDX,
            qm31_const::<694099654, 1271706527, 186094788, 245232983>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            CURR_PROGRAM_27_IDX,
            qm31_const::<626990475, 1137488799, 118985924, 245232983>(),
        );

        let mut trace_columns = [
            [qm31_const::<1659099300, 905558730, 651199673, 1375009625>()].span(),
        ]
            .span();
        let interaction_values = array![
            qm31_const::<1005168032, 79980996, 1847888101, 1941984119>(),
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
            );
        preprocessed_trace.validate_usage();
        assert_eq!(sum, QM31Trait::from_fixed_array(VERIFY_PROGRAM_SEGMENT_SAMPLE_EVAL_RESULT))
    }
}
