pub fn reverse<T: core::cmp::Ord>(a: &T, b: &T) -> core::cmp::Ordering {
    b.cmp(a)
}
