use stwo_prover::core::fields::m31::M31;

// Pads all rows below <padding_offset> with the first row. Uses the <get_m31> function to get the
// value in a given row and column.
pub fn pad<F>(get_m31: F, padding_offset: usize, col: usize) -> Vec<M31>
where
    F: Fn(usize, usize) -> M31,
{
    let n = padding_offset.next_power_of_two();
    (0..n)
        .map(|i| if i < padding_offset { i } else { 0 })
        .map(|i| get_m31(i, col))
        .collect()
}
