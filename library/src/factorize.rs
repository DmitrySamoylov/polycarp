pub fn factorize(mut x: u64) -> Vec<u64> {
    let mut ans = Vec::new();

    for i in 2.. {
        if i * i > x {
            break;
        }

        while x % i == 0 {
            ans.push(i);
            x /= i;
        }
    }

    if x != 1 {
        ans.push(x);
    }

    ans
}

// Visibility: off
#[test]
fn test() {
    assert_eq!(factorize(1), &[]);
    assert_eq!(factorize(2), &[2]);
    assert_eq!(factorize(3), &[3]);
    assert_eq!(factorize(4), &[2, 2]);
    assert_eq!(factorize(28), &[2, 2, 7]);
}
// Visibility: on
