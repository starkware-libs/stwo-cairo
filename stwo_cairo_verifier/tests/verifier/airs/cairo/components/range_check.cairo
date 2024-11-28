use stwo_cairo_verifier::channel::{Channel, ChannelImpl};
use stwo_cairo_verifier::circle::CirclePoint;
use stwo_cairo_verifier::fields::qm31::{QM31, QM31_EXTENSION_DEGREE};
use stwo_cairo_verifier::poly::circle::CanonicCosetImpl;
use stwo_cairo_verifier::utils::ArrayImpl;
use stwo_cairo_verifier::{ColumnArray, TreeArray};
use super::id_to_f252::N_MULTIPLICITY_COLUMNS;
use super::super::utils::U32Impl;

mod rc_19_constraints;
mod rc_4_3_constraints;
mod rc_7_2_5_constraints;
mod rc_9_9_constraints;

#[derive(Drop, Serde, Clone)]
pub struct Claim {
    pub log_ranges: Array<u32>,
}

#[generate_trait]
pub impl ClaimImpl of ClaimTrait {
    fn log_size(self: @Claim) -> u32 {
        let mut sum = 0;

        for log_range in self.log_ranges.span() {
            sum += *log_range;
        };

        sum
    }

    fn log_sizes(self: @Claim) -> TreeArray<Span<u32>> {
        let log_size = self.log_size();
        let preprocessed_log_sizes = array![log_size].span();
        let trace_log_sizes = ArrayImpl::new_repeated(
            self.log_ranges.len() + N_MULTIPLICITY_COLUMNS, log_size,
        )
            .span();
        let interaction_log_sizes = ArrayImpl::new_repeated(QM31_EXTENSION_DEGREE, log_size).span();
        array![preprocessed_log_sizes, trace_log_sizes, interaction_log_sizes]
    }

    fn mix_into(self: @Claim, ref channel: Channel) {
        for log_range in self.log_ranges.span() {
            channel.mix_nonce((*log_range).into());
        };
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
pub struct Rc19BitComponent {
    pub lookup_elements: super::super::RangeCheck19BitElements,
}

#[generate_trait]
pub impl Rc19BitComponentImpl of Rc19BitComponentTrait {
    fn log_size() -> u32 {
        19
    }

    fn mask_points(
        self: @Rc19BitComponent,
        ref trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        let trace_gen = CanonicCosetImpl::new(Self::log_size()).coset.step_size;
        rc_19_constraints::mask_points(
            ref trace_mask_points, ref interaction_trace_mask_points, point, trace_gen,
        );
    }

    fn max_constraint_log_degree_bound(self: @Rc19BitComponent) -> u32 {
        Self::log_size() + 1
    }
}

#[derive(Drop)]
pub struct Rc9Bit9BitComponent {
    pub lookup_elements: super::super::RangeCheck9Bit9BitElements,
}

#[generate_trait]
pub impl Rc9Bit9BitComponentImpl of Rc9Bit9BitComponentTrait {
    fn log_size() -> u32 {
        9 + 9
    }

    fn mask_points(
        self: @Rc9Bit9BitComponent,
        ref trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        let trace_gen = CanonicCosetImpl::new(Self::log_size()).coset.step_size;
        rc_9_9_constraints::mask_points(
            ref trace_mask_points, ref interaction_trace_mask_points, point, trace_gen,
        );
    }

    fn max_constraint_log_degree_bound(self: @Rc9Bit9BitComponent) -> u32 {
        Self::log_size() + 1
    }
}

#[derive(Drop)]
pub struct Rc4Bit3BitComponent {
    pub lookup_elements: super::super::RangeCheck4Bit3BitElements,
}

#[generate_trait]
pub impl Rc4Bit3BitComponentImpl of Rc4Bit3BitComponentTrait {
    fn log_size() -> u32 {
        4 + 3
    }

    fn mask_points(
        self: @Rc4Bit3BitComponent,
        ref trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        let trace_gen = CanonicCosetImpl::new(Self::log_size()).coset.step_size;
        rc_4_3_constraints::mask_points(
            ref trace_mask_points, ref interaction_trace_mask_points, point, trace_gen,
        );
    }

    fn max_constraint_log_degree_bound(self: @Rc4Bit3BitComponent) -> u32 {
        Self::log_size() + 1
    }
}

#[derive(Drop)]
pub struct Rc7Bit2Bit5BitComponent {
    pub lookup_elements: super::super::RangeCheck7Bit2Bit5BitElements,
}

#[generate_trait]
pub impl Rc7Bit2Bit5BitComponentImpl of Rc7Bit2Bit5BitComponentTrait {
    fn log_size() -> u32 {
        7 + 2 + 5
    }

    fn mask_points(
        self: @Rc7Bit2Bit5BitComponent,
        ref trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        let trace_gen = CanonicCosetImpl::new(Self::log_size()).coset.step_size;
        rc_7_2_5_constraints::mask_points(
            ref trace_mask_points, ref interaction_trace_mask_points, point, trace_gen,
        );
    }

    fn max_constraint_log_degree_bound(self: @Rc7Bit2Bit5BitComponent) -> u32 {
        Self::log_size() + 1
    }
}
