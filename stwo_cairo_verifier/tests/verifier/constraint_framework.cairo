use core::dict::Felt252Dict;
use stwo_cairo_verifier::TreeArray;
use stwo_cairo_verifier::channel::{Channel, ChannelImpl};
use stwo_cairo_verifier::fields::m31::M31;
use stwo_cairo_verifier::fields::qm31::{QM31, QM31Impl, QM31One, QM31Zero};

/// Represents the value of the prefix sum column at some index.
/// Should be used to eliminate padded rows for the logup sum.
pub type ClaimedPrefixSum = (QM31, usize);

#[derive(Drop, Clone)]
pub struct LookupElements<const N: usize> {
    pub z: QM31,
    pub alpha: QM31,
    pub alpha_powers: Array<QM31>,
}

#[generate_trait]
pub impl LookupElementsImpl<const N: usize> of LookupElementsTrait<N> {
    fn draw(ref channel: Channel) -> LookupElements<N> {
        assert!(N != 0);
        let [z, alpha]: [QM31; 2] = (*channel.draw_felts(2).span().try_into().unwrap()).unbox();

        let mut acc = QM31One::one();
        let mut alpha_powers = array![acc];

        for _ in 1..N {
            acc *= alpha;
            alpha_powers.append(acc);
        };

        LookupElements { z, alpha, alpha_powers }
    }

    fn combine<impl IntoSpan: ToSpanTrait<[M31; N], M31>>(
        self: @LookupElements<N>, values: [M31; N],
    ) -> QM31 {
        let mut alpha_powers = self.alpha_powers.span();
        let mut values_span = IntoSpan::span(@values);
        let mut sum = -*self.z;

        while let (Option::Some(alpha), Option::Some(value)) =
            (alpha_powers.pop_front(), values_span.pop_front()) {
            sum += (*alpha).mul_m31(*value);
        };

        sum
    }
}

pub struct TraceLocationAllocator {
    /// Mapping of tree index to next available column offset.
    next_tree_offsets: TreeArray<usize>,
    /// Mapping of [`PreprocessedColumn`] to index.
    preprocessed_columns: Felt252Dict<usize>,
    /// Controls whether the preprocessed columns are dynamic or static (default=Dynamic).
    preprocessed_columns_allocation_mode: PreprocessedColumnsAllocationMode,
}

#[generate_trait]
pub impl TraceLocationAllocatorImpl of TraceLocationAllocatorTrait {
    /// Create a new `TraceLocationAllocator` with fixed preprocessed columns setup.
    fn new_with_preproccessed_columns(
        preprocessed_columns: Span<PreprocessedColumn>,
    ) -> TraceLocationAllocator {
        let mut preprocessed_columns_dict = Default::default();
        let mut i = 0;

        for preprocessed_column in preprocessed_columns {
            let key = PreprocessedColumnKey::encode(preprocessed_column);
            preprocessed_columns_dict.insert(key, i);
            i += 1;
        };

        TraceLocationAllocator {
            next_tree_offsets: array![],
            preprocessed_columns: preprocessed_columns_dict,
            preprocessed_columns_allocation_mode: PreprocessedColumnsAllocationMode::Static,
        }
    }
}

#[derive(Debug, Default)]
enum PreprocessedColumnsAllocationMode {
    #[default]
    Dynamic,
    Static,
}

#[derive(Drop, Debug, Copy)]
pub enum PreprocessedColumn {
    /// Symbolic representation of xor lookup table of the form: `(elem_bits, expand_bits, term)`.
    /// Where term is `{ 0 = left operand, 1 = right operand, 2 = xor result }`.
    XorTable: (u32, u32, usize),
    /// Stores the log size of the column.
    IsFirst: u32,
    Plonk: usize,
}

/// An encoding of a [`PreprocessedColumn`] to index into [`Felt252Dict`].
#[generate_trait]
pub impl PreprocessedColumnKey of PreprocessedColumnKeyTrait {
    fn encode(key: @PreprocessedColumn) -> felt252 {
        const FELT252_2_POW_32: felt252 = 0x100000000;
        // TODO: Is there something like Rust's `core::mem::discriminant` in Cairo?
        const XOR_TABLE_DISCRIMINANT: felt252 = 0;
        const IS_FIRST_TABLE_DISCRIMINANT: felt252 = 1;
        const PLONK_TABLE_DISCRIMINANT: felt252 = 2;

        match key {
            PreprocessedColumn::XorTable((
                elem_bits, expand_bits, term,
            )) => {
                let mut res = (*term).into();
                res = res * FELT252_2_POW_32 + (*expand_bits).into();
                res = res * FELT252_2_POW_32 + (*elem_bits).into();
                res = res * FELT252_2_POW_32 + XOR_TABLE_DISCRIMINANT;
                res
            },
            PreprocessedColumn::IsFirst(log_size) => {
                let mut res = (*log_size).into();
                res = res * FELT252_2_POW_32 + IS_FIRST_TABLE_DISCRIMINANT;
                res
            },
            PreprocessedColumn::Plonk(v) => {
                let mut res = (*v).into();
                res = res * FELT252_2_POW_32 + PLONK_TABLE_DISCRIMINANT;
                res
            },
        }
    }
}
