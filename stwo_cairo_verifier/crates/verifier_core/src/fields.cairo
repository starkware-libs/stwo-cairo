pub mod cm31;

pub mod m31;
#[cfg(test)]
mod m31_test;

pub mod qm31;
#[cfg(test)]
mod qm31_test;

pub type BaseField = m31::M31;
pub type SecureField = qm31::QM31;

pub trait Invertible<T> {
    fn inverse(self: T) -> T;
}

// TODO(andrew): Consider removing in favour of inverse libfunc.
pub trait BatchInvertible<T, +Invertible<T>, +Copy<T>, +Drop<T>, +Mul<T>> {
    /// Computes all `1/arr[i]` with a single call to `inverse()` using Montgomery batch inversion.
    fn batch_inverse(
        values: Array<T>,
    ) -> Array<
        T,
    > {
        if values.is_empty() {
            return array![];
        }

        // Collect array of products: `z, z*y, ..., z*y*..*b`.
        let mut suffix_products = array![];
        let mut values_span = values.span();
        let mut cumulative_product = *values_span.pop_back().unwrap();

        while let Some(value) = values_span.pop_back() {
            suffix_products.append(cumulative_product);
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
        let mut suffix_products = suffix_products.span();
        let mut values = values;

        while let (Some(suffix_product), Some(value)) =
            (suffix_products.pop_back(), values.pop_front()) {
            inverses.append(cumulative_product_inv * *suffix_product);
            cumulative_product_inv = cumulative_product_inv * value;
        }

        // Append final `1/z`.
        inverses.append(cumulative_product_inv);

        inverses
    }
}
