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
    array: Span<T>,
    last_index: Option<u32>,
}

trait SortedIteratorTrait<
    T, C, +PartialOrd<T>, +PartialEq<T>, +Copy<T>, +Drop<T>, +Compare<T, C>, +Drop<C>, +Copy<C>
> {
    fn iterate(array_to_iterate: Span<T>) -> SortedIterator<T, C>;

    fn next_deduplicated(
        ref self: SortedIterator<T, C>
    ) -> Option<(u32, T)> {
        next_deduplicated::<T, C>(ref self)
    }

    fn next(
        ref self: SortedIterator<T, C>
    ) -> Option<
        (u32, T)
    > {
        if self.last_index.is_some() {
            let last_index = self.last_index.unwrap();
            let last_value = *self.array[last_index];
            let mut is_repeated = false;

            let mut i = last_index + 1;
            while i < self.array.len() {
                if *self.array[i] == last_value {
                    is_repeated = true;
                    self.last_index = Option::Some(i);
                    break;
                }
                i += 1;
            };

            if is_repeated {
                return Option::Some((self.last_index.unwrap(), last_value));
            }
        }
        next_deduplicated::<T, C>(ref self)
    }
}

fn next_deduplicated<
    T, C, +PartialOrd<T>, +PartialEq<T>, +Copy<T>, +Drop<T>, +Compare<T, C>, +Drop<C>, +Copy<C>
>(
    ref self: SortedIterator<T, C>
) -> Option<(u32, T)> {
    let mut candidate_index = Option::None;
    let mut candidate_value = Option::None;

    let last_value = if let Option::Some(last_index) = self.last_index {
        Option::Some(*self.array[last_index])
    } else {
        Option::None
    };

    let mut i = 0;
    while i < self.array.len() {
        let is_better_than_last = if let Option::Some(last_value) = last_value {
            self.comparer.compare(last_value, *self.array[i])
        } else {
            true
        };
        let is_nearer_than_candidate = if let Option::Some(candidate_value) = candidate_value {
            self.comparer.compare(*self.array[i], candidate_value)
        } else {
            true
        };
        if is_better_than_last && is_nearer_than_candidate {
            candidate_index = Option::Some(i);
            candidate_value = Option::Some(*self.array[i]);
        }
        i += 1;
    };

    if candidate_value.is_none() {
        Option::None
    } else {
        self.last_index = candidate_index;
        Option::Some((candidate_index.unwrap(), candidate_value.unwrap()))
    }
}

pub impl MaximumToMinimumSortedIterator<
    T, +PartialOrd<T>, +PartialEq<T>, +Copy<T>, +Drop<T>
> of SortedIteratorTrait<T, GreaterThan> {
    fn iterate(array_to_iterate: Span<T>) -> SortedIterator<T, GreaterThan> {
        SortedIterator {
            comparer: GreaterThan {}, array: array_to_iterate, last_index: Option::None
        }
    }
}

pub impl MinimumToMaximumSortedIterator<
    T, +PartialOrd<T>, +PartialEq<T>, +Copy<T>, +Drop<T>
> of SortedIteratorTrait<T, LowerThan> {
    fn iterate(array_to_iterate: Span<T>) -> SortedIterator<T, LowerThan> {
        SortedIterator { comparer: LowerThan {}, array: array_to_iterate, last_index: Option::None }
    }
}


#[test]
fn test_sort_lowest_to_greatest() {
    let my_array: Array<u32> = array![3, 5, 2, 4];
    let expected_array: Array<u32> = array![2, 3, 4, 5];

    let mut sorted_array = array![];

    let mut iterator = MinimumToMaximumSortedIterator::iterate(my_array.span());
    while let Option::Some((_index, value)) = iterator.next_deduplicated() {
        sorted_array.append(value);
    };

    assert_eq!(expected_array, sorted_array);
}

#[test]
fn test_sort_greatest_to_lowest() {
    let my_array: Array<u32> = array![3, 5, 2, 4];
    let expected_array: Array<u32> = array![5, 4, 3, 2];

    let mut sorted_array = array![];

    let mut iterator = MaximumToMinimumSortedIterator::iterate(my_array.span());
    while let Option::Some((_index, value)) = iterator.next_deduplicated() {
        sorted_array.append(value);
    };

    assert_eq!(expected_array, sorted_array);
}

#[test]
fn test_sort_indexes_are_correct() {
    let my_array: Array<u32> = array![3, 5, 2, 4];
    let expected_indexes: Array<u32> = array![2, 0, 3, 1];

    let mut sorted_indexes = array![];

    let mut iterator = MinimumToMaximumSortedIterator::iterate(my_array.span());
    while let Option::Some((index, _value)) = iterator.next_deduplicated() {
        sorted_indexes.append(index);
    };

    assert_eq!(expected_indexes, sorted_indexes);
}

#[test]
fn test_sort_with_duplicates() {
    let my_array: Array<u32> = array![3, 5, 2, 3, 4, 3, 4];
    let expected_indexes: Array<u32> = array![2, 0, 3, 5, 4, 6, 1];
    let expected_array: Array<u32> = array![2, 3, 3, 3, 4, 4, 5];

    let mut sorted_indexes = array![];
    let mut sorted_array = array![];

    let mut iterator = MinimumToMaximumSortedIterator::iterate(my_array.span());
    while let Option::Some((index, value)) = iterator.next() {
        sorted_array.append(value);
        sorted_indexes.append(index);
    };

    assert_eq!(expected_indexes, sorted_indexes);
    assert_eq!(expected_array, sorted_array);
}

