pub mod cm31;
pub mod m31;
pub mod qm31;

pub type BaseField = m31::M31;
pub type SecureField = qm31::QM31;

pub trait Invertible<T> {
    fn inverse(self: T) -> T;
}

// TODO(andrew): Consider removing in favour of inverse libfunc.
pub trait BatchInvertible<T, +Invertible<T>, +Copy<T>, +Drop<T>, +Mul<T>> {
    /// Computes all `1/arr[i]` with a single call to `inverse()` using Montgomery batch inversion.
    fn batch_inverse(values: Array<T>) -> Array<T> {
        if values.is_empty() {
            return array![];
        }

        // Collect array of products: `z, z*y, ..., z*y*..*b`.
        // rename to suffix_products
        let mut prefix_product_rev = array![];
        let mut values_span = values.span();
        let mut cumulative_product = *values_span.pop_back().unwrap();

        while let Some(value) = values_span.pop_back() {
            prefix_product_rev.append(cumulative_product);
            cumulative_product = cumulative_product * *value;
        }

        // Compute `1/(z*y*..*b*a)`.
        let mut cumulative_product_inv = cumulative_product.inverse();

        // Collect all:
        //   `1/a = (z*y*..*b)/(z*y*..*a)`,
        //   `1/b = (z*y*..*c)/(z*y*..*b)`,
        //   ...,
        //   `1/y = z/(z*y)`.
        let mut inverses = array![];
        let mut prefix_product_rev_span = prefix_product_rev.span();
        let mut values = values;

        while let (Some(prefix_product), Some(value)) =
            (prefix_product_rev_span.pop_back(), values.pop_front()) {
            inverses.append(cumulative_product_inv * *prefix_product);
            cumulative_product_inv = cumulative_product_inv * value;
        }

        // Append final `1/z`.
        inverses.append(cumulative_product_inv);

        inverses
    }
}

// in general, move tests to separate files

#[cfg(test)]
mod tests {
    use super::m31::{M31, m31};
    use super::{BatchInvertible, Invertible};

    #[test]
    fn test_batch_inverse() {
        let arr = array![m31(2), m31(3), m31(5), m31(7)];
        let mut arr_inv = array![];
        for v in arr.span() {
            arr_inv.append((*v).inverse());
        }

        let res = BatchInvertible::batch_inverse(arr);

        assert_eq!(res, arr_inv);
    }

    #[test]
    fn test_batch_inverse_with_empty_array() {
        let arr: Array<M31> = array![];

        let res = BatchInvertible::batch_inverse(arr);

        assert_eq!(res, array![]);
    }

    #[test]
    fn test_batch_inverse_with_single_value() {
        let two = m31(2);
        let two_inv = two.inverse();
        let arr = array![two];

        let res = BatchInvertible::batch_inverse(arr);

        assert_eq!(res, array![two_inv]);
    }
}
