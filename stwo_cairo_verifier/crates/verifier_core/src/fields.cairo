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

        // Collect array `z, zy, ..., zy..b`.
        let mut prefix_product_rev = array![];
        let mut values_span = values.span();
        let mut cumulative_product = *values_span.pop_back().unwrap();

        while let Some(value) = values_span.pop_back() {
            prefix_product_rev.append(cumulative_product);
            cumulative_product = cumulative_product * *value;
        }

        // Compute `1/zy..a`.
        let mut cumulative_product_inv = cumulative_product.inverse();

        // Collect all `1/a = zy..b/zy..a, 1/b = zy..c/zy..b, ..., 1/y = z/zy`.
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