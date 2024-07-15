use stwo_cairo_verifier::fields::SecureField;


pub fn fold(
    values: @Array<SecureField>,
    folding_factors: @Array<SecureField>,
    index: usize,
    level: usize,
    n: usize
) -> SecureField {
    if n == 1 {
        return *values[index];
    }

    let lhs_val = fold(values, folding_factors, index, level + 1, n / 2);
    let rhs_val = fold(values, folding_factors, index + n / 2, level + 1, n / 2);
    return lhs_val + rhs_val * *folding_factors[level];
}
