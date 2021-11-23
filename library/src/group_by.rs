// https://github.com/rust-lang/rust/issues/80552
trait MyGroupByTrait<T> {
    fn my_group_by<P>(&self, predicate: P) -> MyGroupBy<'_, T, P>
    where
        P: Fn(&T, &T) -> bool;
}

pub struct MyGroupBy<'a, T: 'a, P>
where
    P: Fn(&T, &T) -> bool,
{
    slice: &'a [T],
    predicate: P,
}

impl<T> MyGroupByTrait<T> for &[T] {
    fn my_group_by<P>(&self, predicate: P) -> MyGroupBy<'_, T, P>
    where
        P: Fn(&T, &T) -> bool,
    {
        MyGroupBy {
            slice: self,
            predicate,
        }
    }
}

impl<T> MyGroupByTrait<T> for Vec<T> {
    fn my_group_by<P>(&self, predicate: P) -> MyGroupBy<'_, T, P>
    where
        P: Fn(&T, &T) -> bool,
    {
        MyGroupBy {
            slice: self.as_slice(),
            predicate,
        }
    }
}

impl<'a, T: 'a, P> Iterator for MyGroupBy<'a, T, P>
where
    P: Fn(&T, &T) -> bool,
{
    type Item = &'a [T];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.slice.is_empty() {
            None
        } else {
            let mut len = 1;
            let mut iter = self.slice.windows(2);
            while let Some([l, r]) = iter.next() {
                if (self.predicate)(l, r) {
                    len += 1
                } else {
                    break;
                }
            }
            let (head, tail) = self.slice.split_at(len);
            self.slice = tail;
            Some(head)
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.slice.is_empty() {
            (0, Some(0))
        } else {
            (1, Some(self.slice.len()))
        }
    }

    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}

impl<'a, T: 'a, P> DoubleEndedIterator for MyGroupBy<'a, T, P>
where
    P: Fn(&T, &T) -> bool,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.slice.is_empty() {
            None
        } else {
            let mut len = 1;
            let mut iter = self.slice.windows(2);
            while let Some([l, r]) = iter.next_back() {
                if (self.predicate)(l, r) {
                    len += 1
                } else {
                    break;
                }
            }
            let (head, tail) = self.slice.split_at(self.slice.len() - len);
            self.slice = head;
            Some(tail)
        }
    }
}

// Visibility: off
#[test]
fn test_freq() {
    let v = vec![1, 2, 2, 3, 4, 4, 4];

    let f = v
        .my_group_by(PartialEq::eq)
        .map(|g| g.len())
        .collect::<Vec<_>>();

    assert_eq!(f, [1, 2, 1, 3]);
}

#[test]
fn test_cmp() {
    let v = vec![1, 2, 2, 4, 4, 4, 5, 5];
    let f = v
        .my_group_by(|a, b| a % 2 == b % 2)
        .map(|g| g.len())
        .collect::<Vec<_>>();

    assert_eq!(f, [1, 5, 2]);
}
// Visibility: on
