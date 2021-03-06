pub fn divisors(x: u64) -> Vec<u64> {
    let mut ans = Vec::new();

    for i in 1.. {
        if i * i > x {
            break;
        }

        if x % i == 0 {
            if x / i == i {
                ans.push(i);
            } else {
                ans.push(i);
                ans.push(x / i);
            }
        }
    }

    ans.sort_unstable();

    ans
}

// Visibility: off
#[test]
fn test() {
    assert_eq!(divisors(1), &[1]);
    assert_eq!(divisors(4), &[1, 2, 4]);
    assert_eq!(divisors(6), &[1, 2, 3, 6]);
    assert_eq!(divisors(16), &[1, 2, 4, 8, 16]);
    assert_eq!(divisors(19), &[1, 19]);
}
// Visibility: on
