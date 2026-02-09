use std::sync::Arc;

use stwo_cairo_adapter::builtins::BuiltinSegments;
use stwo_cairo_common::builtins::{
    ADD_MOD_BUILTIN_MEMORY_CELLS, BITWISE_BUILTIN_MEMORY_CELLS, MUL_MOD_BUILTIN_MEMORY_CELLS,
    PEDERSEN_BUILTIN_MEMORY_CELLS, POSEIDON_BUILTIN_MEMORY_CELLS,
    RANGE_CHECK_96_BUILTIN_MEMORY_CELLS, RANGE_CHECK_BUILTIN_MEMORY_CELLS,
};
use stwo_cairo_common::preprocessed_columns::preprocessed_trace::PreProcessedTrace;
use stwo_constraint_framework::preprocessed_columns::PreProcessedColumnId;

pub fn get_builtins(
    builtin_segments: &BuiltinSegments,
    preprocessed_trace: Arc<PreProcessedTrace>,
) -> Vec<&'static str> {
    let mut builtins = vec![];

    if let Some(segment) = builtin_segments.add_mod_builtin {
        let segment_length = segment.stop_ptr - segment.begin_addr;
        assert!(
            segment_length.is_multiple_of(ADD_MOD_BUILTIN_MEMORY_CELLS),
            "add mod segment length is not a multiple of it's cells_per_instance"
        );
        let n_instances = segment_length / ADD_MOD_BUILTIN_MEMORY_CELLS;
        assert!(
            n_instances.is_power_of_two(),
            "add mod instances number is not a power of two"
        );
        builtins.push("add_mod_builtin");
    }
    if let Some(segment) = builtin_segments.bitwise_builtin {
        let segment_length = segment.stop_ptr - segment.begin_addr;
        assert!(
            segment_length.is_multiple_of(BITWISE_BUILTIN_MEMORY_CELLS),
            "bitwise segment length is not a multiple of it's cells_per_instance"
        );
        let n_instances = segment_length / BITWISE_BUILTIN_MEMORY_CELLS;
        assert!(
            n_instances.is_power_of_two(),
            "bitwise instances number is not a power of two"
        );
        builtins.push("bitwise_builtin");
    }
    if let Some(segment) = builtin_segments.mul_mod_builtin {
        let segment_length = segment.stop_ptr - segment.begin_addr;
        assert!(
            segment_length.is_multiple_of(MUL_MOD_BUILTIN_MEMORY_CELLS),
            "mul mod segment length is not a multiple of it's cells_per_instance"
        );
        let n_instances = segment_length / MUL_MOD_BUILTIN_MEMORY_CELLS;
        assert!(
            n_instances.is_power_of_two(),
            "mul mod instances number is not a power of two"
        );
        builtins.push("mul_mod_builtin");
    }
    if let Some(segment) = builtin_segments.pedersen_builtin {
        let segment_length = segment.stop_ptr - segment.begin_addr;
        assert!(
            segment_length.is_multiple_of(PEDERSEN_BUILTIN_MEMORY_CELLS),
            "pedersen segment length is not a multiple of it's cells_per_instance"
        );
        let n_instances = segment_length / PEDERSEN_BUILTIN_MEMORY_CELLS;
        assert!(
            n_instances.is_power_of_two(),
            "pedersen instances number is not a power of two"
        );
        if preprocessed_trace.has_column(&PreProcessedColumnId {
            id: "pedersen_points_0".to_owned(),
        }) {
            builtins.push("pedersen_builtin");
        } else if preprocessed_trace.has_column(&PreProcessedColumnId {
            id: "pedersen_points_small_0".to_owned(),
        }) {
            builtins.push("pedersen_builtin_narrow_windows");
        } else {
            panic!("Missing pedersen points in the preprocessed trace.")
        }
    }
    if let Some(segment) = builtin_segments.poseidon_builtin {
        let segment_length = segment.stop_ptr - segment.begin_addr;
        assert!(
            segment_length.is_multiple_of(POSEIDON_BUILTIN_MEMORY_CELLS),
            "poseidon segment length is not a multiple of it's cells_per_instance"
        );
        let n_instances = segment_length / POSEIDON_BUILTIN_MEMORY_CELLS;
        assert!(
            n_instances.is_power_of_two(),
            "poseidon instances number is not a power of two"
        );
        builtins.push("poseidon_builtin");
    }
    if let Some(segment) = builtin_segments.range_check96_builtin {
        let segment_length = segment.stop_ptr - segment.begin_addr;
        assert!(
            segment_length.is_multiple_of(RANGE_CHECK_96_BUILTIN_MEMORY_CELLS),
            "range_check96 segment length is not a multiple of it's cells_per_instance"
        );
        let n_instances = segment_length / RANGE_CHECK_96_BUILTIN_MEMORY_CELLS;
        assert!(
            n_instances.is_power_of_two(),
            "range_check96 instances number is not a power of two"
        );
        builtins.push("range_check96_builtin");
    }
    if let Some(segment) = builtin_segments.range_check_builtin {
        let segment_length = segment.stop_ptr - segment.begin_addr;
        assert!(
            segment_length.is_multiple_of(RANGE_CHECK_BUILTIN_MEMORY_CELLS),
            "range_check segment length is not a multiple of it's cells_per_instance"
        );
        let n_instances = segment_length / RANGE_CHECK_BUILTIN_MEMORY_CELLS;
        assert!(
            n_instances.is_power_of_two(),
            "range_check instances number is not a power of two"
        );
        builtins.push("range_check_builtin");
    }

    builtins
}
