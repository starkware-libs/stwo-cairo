pub mod component;

// Used for getting array sizes as consts.
#[macro_export]
macro_rules! count_elements {
    ($x:expr) => (1);
    ($x:expr, $($xs:expr),*) => (1 + $crate::count_elements!($($xs),*));
}

#[macro_export]
macro_rules! generate_range_check_constraints {
    ([$($log_range:expr),+]) => {
        paste::paste!{
            pub mod [<range_check_$($log_range)_*>] {
                $crate::range_check_eval!($($log_range),+);
            }
        }
    };
}

generate_range_check_constraints!([6]);
generate_range_check_constraints!([8]);
generate_range_check_constraints!([11]);
generate_range_check_constraints!([12]);
generate_range_check_constraints!([18]);
generate_range_check_constraints!([19]);
generate_range_check_constraints!([3, 6]);
generate_range_check_constraints!([4, 3]);
generate_range_check_constraints!([4, 4]);
generate_range_check_constraints!([5, 4]);
generate_range_check_constraints!([9, 9]);
generate_range_check_constraints!([7, 2, 5]);
generate_range_check_constraints!([3, 6, 6, 3]);
generate_range_check_constraints!([4, 4, 4, 4]);
generate_range_check_constraints!([3, 3, 3, 3, 3]);

#[cfg(test)]
mod tests {
    use stwo_prover::core::backend::simd::column::BaseColumn;
    use stwo_prover::core::fields::m31::M31;

    use crate::witness::components::range_check_vector::generate_partitioned_enumeration;

    #[test]
    fn test_packed_partition_enumerate() {
        let log_ranges = [5, 3, 3];
        let mut expected = [vec![], vec![], vec![]];
        for i in 0..1 << 5 {
            for j in 0..1 << 3 {
                for k in 0..1 << 3 {
                    expected[0].push(M31(i));
                    expected[1].push(M31(j));
                    expected[2].push(M31(k));
                }
            }
        }

        let mut result = generate_partitioned_enumeration(log_ranges).into_iter();
        let result: [Vec<M31>; 3] =
            std::array::from_fn(|_| BaseColumn::from_simd(result.next().unwrap()).into_cpu_vec());

        assert_eq!(result, expected)
    }
}
