use starknet_types_core::felt::Felt;

pub fn felt_batch_inverse(values: &[Felt]) -> Vec<Felt> {
    let mut partial_prods = Vec::with_capacity(values.len()); // partial_prods[k] is the product of the first <k> elements
    let mut prod = Felt::ONE;
    for value in values.iter() {
        partial_prods.push(prod);
        prod *= value;
    }

    let mut partial_inverse = prod.inverse().expect("Input values should be invertible");

    let mut inverses_reversed = Vec::with_capacity(values.len());

    for i in 0..values.len() {
        // Loop invariant: partial_inverse is the inverse of the first N - i elements
        let inverse = partial_inverse * partial_prods[values.len() - i - 1];
        inverses_reversed.push(inverse);
        partial_inverse *= values[values.len() - i - 1];
    }

    inverses_reversed.reverse();
    inverses_reversed
}

mod tests {
    #[cfg(test)]
    use starknet_types_core::felt::Felt;

    #[cfg(test)]
    use crate::preprocessed_columns::felt_batch_inverse::felt_batch_inverse;

    #[test]
    fn test_batch_inverse() {
        assert_eq!(felt_batch_inverse(&[1.into()]), [1.into()]);
        assert_eq!(
            felt_batch_inverse(&[Felt::from(17).inverse().unwrap()]),
            [17.into()]
        );
        assert_eq!(
            felt_batch_inverse(&(1..100).map(Felt::from).collect::<Vec<_>>()),
            (1..100)
                .map(|x| Felt::from(x).inverse().unwrap())
                .collect::<Vec<_>>()
        );
        assert_eq!(
            felt_batch_inverse(
                &(1..100)
                    .map(|x| Felt::from(x).inverse().unwrap())
                    .collect::<Vec<_>>()
            ),
            (1..100).map(Felt::from).collect::<Vec<_>>()
        );
    }
}
