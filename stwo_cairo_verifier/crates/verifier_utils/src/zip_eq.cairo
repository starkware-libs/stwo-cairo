/// An iterator that iterates two other iterators simultaneously, panics if the iterators have
/// different lengths.
///
/// This struct is created by [`zip_eq`].
///
/// Unlike [`Zip`], this iterator ensures that both input iterators have exactly the same length.
/// If one iterator is exhausted while the other still has elements, the iterator will panic.
#[derive(Drop, Clone)]
#[must_use]
pub struct ZipEq<A, B> {
    a: A,
    b: B,
}

#[inline]
pub fn zip_eq<
    A,
    B,
    impl AIntoIter: IntoIterator<A>,
    impl BIntoIter: IntoIterator<B>,
    +Destruct<AIntoIter::IntoIter>,
    +Destruct<B>,
>(
    a: A, b: B,
) -> ZipEq<AIntoIter::IntoIter, BIntoIter::IntoIter> {
    ZipEq { a: a.into_iter(), b: b.into_iter() }
}

pub impl ZipEqIterator<
    A,
    B,
    impl IterA: Iterator<A>,
    impl IterB: Iterator<B>,
    +Destruct<A>,
    +Destruct<B>,
    +Destruct<IterA::Item>,
    +Destruct<IterB::Item>,
> of Iterator<ZipEq<A, B>> {
    type Item = (IterA::Item, IterB::Item);

    #[inline]
    fn next(ref self: ZipEq<A, B>) -> Option<Self::Item> {
        let Some(a) = self.a.next() else {
            if let Some(_) = self.b.next() {
                // Note that we unpack 'b' here because we don't have a
                // Destruct<Option<IterB::Item>>.
                panic!("ZipEq: iterators have different lengths");
            }
            return None;
        };
        let Some(b) = self.b.next() else {
            panic!("ZipEq: iterators have different lengths");
        };
        Some((a, b))
    }
}
