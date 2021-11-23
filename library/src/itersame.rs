pub fn itersame_cmp<T, F: FnMut(&[T]), C: Fn(&T, &T) -> bool>(mut slice: &[T], mut cb: F, cmp: C) {
    while !slice.is_empty() {
        let pos = slice
            .windows(2)
            .position(|w| !cmp(&w[0], &w[1]))
            .map(|pos| pos + 1)
            .unwrap_or_else(|| slice.len());
        let (head, tail) = slice.split_at(pos);
        cb(head);
        slice = tail;
    }
}

pub fn itersame<T: PartialEq, F: FnMut(&[T])>(x: &[T], foreach: F) {
    itersame_cmp(x, foreach, PartialEq::eq)
}

// Visibility: off
#[test]
fn test_freq() {
    let v = vec![1, 2, 2, 3, 4, 4, 4];
    let mut f = vec![];

    itersame(&v, |s| f.push(s.len()));

    assert_eq!(f, [1, 2, 1, 3]);
}

#[test]
fn test_empty() {
    let v = Vec::<i32>::new();
    let mut seen = false;

    itersame(&v, |_| seen = true);

    assert!(!seen);
}

#[test]
fn test_one() {
    let v = vec![42];
    let mut f = vec![];

    itersame(&v, |s| f.push(s.len()));

    assert_eq!(f, [1]);
}

#[test]
fn test_all_different() {
    let v = (0..5).collect::<Vec<_>>();
    let mut f = vec![];

    itersame(&v, |s| f.push(s.len()));

    assert_eq!(f, [1; 5]);
}

#[test]
fn test_cmp() {
    let v = vec![1, 2, 2, 4, 4, 4, 5, 5];
    let mut f = vec![];

    itersame_cmp(&v, |s| f.push(s.len()), |a, b| a % 2 == b % 2);

    assert_eq!(f, [1, 5, 2]);
}
// Visibility: on
