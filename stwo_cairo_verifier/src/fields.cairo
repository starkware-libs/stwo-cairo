pub mod cm31;
pub mod m31;
pub mod qm31;

pub type BaseField = m31::M31;
pub type SecureField = qm31::QM31;

pub trait Field<T> {
    fn inverse(self: T) -> T;
}

pub trait FieldBatchInverse<T, +Field<T>, +Copy<T>, +Drop<T>, +Mul<T>> {
    /// Computes all `1/arr[i]` with a single call to `inverse()` using Montgomery batch inversion.
    fn batch_inverse(
        arr: Array<T>
    ) -> Array<
        T
    > {
        if arr.is_empty() {
            return array![];
        }

        // Collect array `z, zy, ..., zy..b`.
        let mut prefix_product_rev = array![];
        let mut cumulative_product = *arr[arr.len() - 1];

        let mut i = arr.len() - 1;
        while i != 0 {
            i -= 1;
            prefix_product_rev.append(cumulative_product);
            cumulative_product = cumulative_product * *arr[i];
        };

        // Compute `1/zy..a`.
        let mut cumulative_product_inv = cumulative_product.inverse();
        // Collect all `1/a = zy..b/zy..a, 1/b = zy..c/zy..b, ..., 1/y = z/zy`.
        let mut inverses = array![];
        let mut arr = arr;

        let mut i = prefix_product_rev.len();
        while i != 0 {
            i -= 1;
            inverses.append(cumulative_product_inv * *prefix_product_rev[i]);
            cumulative_product_inv = cumulative_product_inv * arr.pop_front().unwrap();
        };

        // Append final `1/z`.
        inverses.append(cumulative_product_inv);

        inverses
    }
}

#[cfg(test)]
mod tests {
    use super::m31::{M31, m31};
    use super::{Field, FieldBatchInverse};

    #[test]
    fn test_batch_inverse() {
        let arr = array![m31(2), m31(3), m31(5), m31(7)];
        let mut arr_inv = array![];
        for v in arr.span() {
            arr_inv.append((*v).inverse());
        };

        let res = FieldBatchInverse::batch_inverse(arr);

        assert_eq!(res, arr_inv);
    }

    #[test]
    fn test_batch_inverse_with_empty_array() {
        let arr: Array<M31> = array![];

        let res = FieldBatchInverse::batch_inverse(arr);

        assert_eq!(res, array![]);
    }

    #[test]
    fn test_batch_inverse_with_single_value() {
        let two = m31(2);
        let two_inv = two.inverse();
        let arr = array![two];

        let res = FieldBatchInverse::batch_inverse(arr);

        assert_eq!(res, array![two_inv]);
    }
}
