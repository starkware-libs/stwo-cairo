use core::array::ToSpanTrait;
use core::array::ArrayTrait;
use core::option::OptionTrait;

trait Compare<T, C> {
    fn compare(self: @C, a: T, b: T) -> bool;
}

#[derive(Drop, Copy)]
pub struct LowerThan {}

impl LowerThanCompare<T, +PartialOrd<T>> of Compare<T, LowerThan> {
    fn compare(self: @LowerThan, a: T, b: T) -> bool {
        return a < b;
    }
}

#[derive(Drop, Copy)]
pub struct GreaterThan {}

impl GreaterThanCompare<T, +PartialOrd<T>, +Copy<T>, +Drop<T>> of Compare<T, GreaterThan> {
    fn compare(self: @GreaterThan, a: T, b: T) -> bool {
        return a > b;
    }
}

#[derive(Drop)]
pub struct SortedIterator<T, C> {
    comparer: C,
    current_bound: Option<T>,
}

trait SortedIteratorTrait<T, C, +PartialOrd<T>, +Copy<T>, +Drop<T>, +Compare<T, C>, +Drop<C>, +Copy<C>> {
    fn new() -> SortedIterator<T, C>;

    fn iterate(ref self: SortedIterator<T, C>, array: Span<T>) -> Option<(T, u32)> {
        let mut candidate_value = Option::None;
        let mut candidate_index = Option::None;
    
        let mut i = 0;
        while i < array.len() {
            let bound_condition = if let Option::Some(current_bound) = self.current_bound {
                self.comparer.compare(current_bound, *array[i])
            } else {
                true
            };
            let is_better_than_candidate = if let Option::Some(candidate_value) = candidate_value {
                self.comparer.compare(*array[i], candidate_value)
            } else {
                true
            };
            if bound_condition && is_better_than_candidate {
                candidate_value = Option::Some(*array[i]);
                candidate_index = Option::Some(i);
            }
            i += 1;
        };
    
        if(candidate_value.is_none()) {
            Option::None
        } else {
            self.current_bound = candidate_value;
            Option::Some((candidate_value.unwrap(), candidate_index.unwrap()))
        }    
    }
}

impl MaximumToMinimumSortedIterator<T, +PartialOrd<T>, +Copy<T>, +Drop<T>> of SortedIteratorTrait<T, GreaterThan> {
    fn new() -> SortedIterator<T, GreaterThan> {
        SortedIterator { comparer: GreaterThan {}, current_bound: Option::None }
    }
}

impl MinimumToMaximumSortedIterator<T, +PartialOrd<T>, +Copy<T>, +Drop<T>> of SortedIteratorTrait<T, LowerThan> {
    fn new() -> SortedIterator<T, LowerThan> {
        SortedIterator { comparer: LowerThan {}, current_bound: Option::None }
    }
}

// Returns the element in `arr` that is nearest to `bound` according to the comparer criterion
pub fn iterate_sorted<C, +Compare<u32, C>>(arr: Span<u32>, ref bound: Option<u32>, comparer: @C)
 -> Option<(u32, u32)> {
    let mut candidate_value = Option::None;
    let mut candidate_index = Option::None;

    let mut i = 0;
    while i < arr.len() {
        let bound_condition = if let Option::Some(bound) = bound {
            comparer.compare(bound, *arr[i])
        } else {
            true
        };
        let is_better_than_candidate = if let Option::Some(candidate_value) = candidate_value {
            comparer.compare(*arr[i], candidate_value)
        } else {
            true
        };
        if bound_condition && is_better_than_candidate {
            candidate_value = Option::Some(*arr[i]);
            candidate_index = Option::Some(i);
        }
        i += 1;
    };

    if(candidate_value.is_none()) {
        Option::None
    } else {
        bound = candidate_value;
        Option::Some((candidate_value.unwrap(), candidate_index.unwrap()))
    }
}

#[test]
fn test_sort_lowest_to_greatest() {
    let my_array: Array<u32> = array![3, 5, 2, 4];
    let expected_array: Array<u32> = array![2, 3, 4, 5];

    let mut sorted_array = array![];

    let mut iterator = MinimumToMaximumSortedIterator::new();
    while let Option::Some((value, _index)) = iterator.iterate(my_array.span()) {
        sorted_array.append(value);
    };

    assert_eq!(expected_array, sorted_array);
}

#[test]
fn test_sort_greatest_to_lowest() {
    let my_array: Array<u32> = array![3, 5, 2, 4];
    let expected_array: Array<u32> = array![5, 4, 3, 2];

    let mut sorted_array = array![];

    let mut iterator = MaximumToMinimumSortedIterator::new();
    while let Option::Some((value, _index)) = iterator.iterate(my_array.span()) {
        sorted_array.append(value);
    };

    assert_eq!(expected_array, sorted_array);
}

#[test]
fn test_sort_indexes_are_correct() {
    let my_array: Array<u32> = array![3, 5, 2, 4];
    let expected_indexes: Array<u32> = array![2, 0, 3, 1];

    let mut sorted_indexes = array![];

    let mut iterator = MinimumToMaximumSortedIterator::new();
    while let Option::Some((_value, index)) = iterator.iterate(my_array.span()) {
        sorted_indexes.append(index);
    };

    assert_eq!(expected_indexes, sorted_indexes);
}

#[test]
fn test_sort_greatest_to_lowest_iterator() {
    let my_array: Array<u32> = array![3, 5, 2, 4];
    let expected_array: Array<u32> = array![5, 4, 3, 2];

    let mut sorted_array = array![];

    let mut iterator = MaximumToMinimumSortedIterator::new();
    while let Option::Some((value, _index)) = iterator.iterate(my_array.span()) {
        sorted_array.append(value);
    };

    assert_eq!(expected_array, sorted_array);
}
