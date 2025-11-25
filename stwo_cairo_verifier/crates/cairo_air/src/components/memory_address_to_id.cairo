use crate::prelude::*;

mod constraints;

/// Split the (ID , Multiplicity) columns to shorter chunks. This is done to improve the performance
/// during The Merkle commitment and FRI, as this component is usually the tallest in the Cairo AIR.
///
/// 1. The ID and Multiplicity vectors are split to 'MEMORY_ADDRESS_TO_ID_SPLIT' chunks of size
///    `ids.len()`/`MEMORY_ADDRESS_TO_ID_SPLIT`.
/// 2. The chunks are padded with 0s to the next power of 2.
///
/// #  Example
/// ID = [id0..id10], MEMORY_ADDRESS_TO_ID_SPLIT = 4:
/// ID0 = [id0, id1, id2, 0]
/// ID1 = [id3, id4, id5, 0]
/// ID2 = [id6, id7, id8, 0]
/// ID3 = [id9, id10, 0, 0]
pub const LOG_MEMORY_ADDRESS_TO_ID_SPLIT: u32 = 4;
pub const MEMORY_ADDRESS_TO_ID_SPLIT: usize = 16;
pub const N_ID_AND_MULT_COLUMNS_PER_CHUNK: usize = 2;
pub const N_TRACE_COLUMNS: usize = MEMORY_ADDRESS_TO_ID_SPLIT * N_ID_AND_MULT_COLUMNS_PER_CHUNK;

// Number of QM31 columns in the interaction trace.
// Each QM31 column is implemented as 4 M31 columns.
pub const N_INTERACTION_TRACE_QM31_COLUMNS: usize = (MEMORY_ADDRESS_TO_ID_SPLIT / 2);

pub const RELATION_USES_PER_ROW: [(felt252, u32); 0] = [];

#[derive(Drop, Serde, Copy)]
pub struct Claim {
    pub log_size: u32,
}

pub impl ClaimImpl of ClaimTrait<Claim> {
    fn log_sizes(self: @Claim) -> TreeArray<Span<u32>> {
        let log_size = *self.log_size;
        let preprocessed_log_sizes = array![log_size].span();
        let trace_log_sizes = [log_size; N_TRACE_COLUMNS].span();
        let interaction_log_sizes = [
            log_size
        ; N_INTERACTION_TRACE_QM31_COLUMNS * QM31_EXTENSION_DEGREE]
            .span();
        array![preprocessed_log_sizes, trace_log_sizes, interaction_log_sizes]
    }

    fn mix_into(self: @Claim, ref channel: Channel) {
        channel.mix_u64((*self.log_size).into());
    }

    fn accumulate_relation_uses(self: @Claim, ref relation_uses: RelationUsesDict) {}
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
    pub lookup_elements: crate::MemoryAddressToIdElements,
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
            lookup_elements: interaction_elements.memory_address_to_id.clone(),
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
        let log_size = *self.claim.log_size;

        let params = constraints::ConstraintParams {
            column_size: pow2(log_size).try_into().unwrap(),
            lookup_elements: self.lookup_elements,
            seq: preprocessed_mask_values
                .get_and_mark_used(preprocessed_columns::seq_column_idx(log_size)),
            claimed_sum: *self.interaction_claim.claimed_sum,
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
