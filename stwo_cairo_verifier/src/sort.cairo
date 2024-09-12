
trait Compare<T> {
    fn compare(self: @T, a: u32, b: u32) -> bool;
}

#[derive(Drop)]
pub struct LowerThan {}

pub impl LowerThanCompare of Compare<LowerThan> {
    fn compare(self: @LowerThan, a: u32, b: u32) -> bool {
        return a < b;
    }
}

#[derive(Drop)]
pub struct GreaterThan {}

pub impl GreaterThanCompare of Compare<GreaterThan> {
    fn compare(self: @GreaterThan, a: u32, b: u32) -> bool {
        return a < b;
    }
}

// Returns the element in `arr` that is nearest to `bound` according to the comparer criterion
pub fn iterate_sorted<T, impl TCompare: Compare<T>>(arr: @Array<u32>, upper_bound: Option<u32>, comparer: @T) -> (Option<u32>, Option<u32>) {
    let mut maximum = Option::None;
    let mut index = Option::None;

    let mut i = 0;
    while i < arr.len() {
        let upper_bound_condition = if let Option::Some(upper_bound) = upper_bound {
            comparer.compare(upper_bound, *arr[i])
        } else {
            true
        };
        let lower_bound_condition = if let Option::Some(maximum) = maximum {
            comparer.compare(*arr[i], maximum)
        } else {
            true
        };
        if upper_bound_condition && lower_bound_condition {
            maximum = Option::Some(*arr[i]);
            index = Option::Some(i);
        }
        i += 1;
    };
    (maximum, index)
}