pub fn calc(mut f0: u64, mut f1: u64, n: u64) -> u64 {
    if n == 1 {
        return f0;
    }

    for _ in 2..n {
        let t = f1;
        f1 = f1 + f0;
        f0 = t;
    }

    f1
}

pub fn calc_mod(mut f0: u64, mut f1: u64, n: u64, modl: u64) -> u64 {
    if n == 1 {
        return f0;
    }

    for _ in 2..n {
        let t = f1;
        f1 = (f1 + f0) % modl;
        f0 = t;
    }

    f1
}

#[test]
fn test() {
    assert_eq!(calc(1, 2, 1), 1);
    assert_eq!(calc(1, 2, 2), 2);
    assert_eq!(calc(1, 2, 3), 3);
    assert_eq!(calc(1, 2, 4), 5);
    assert_eq!(calc(1, 2, 5), 8);
    assert_eq!(calc(1, 2, 6), 13);
}

#[test]
fn test_mod() {
    assert_eq!(calc_mod(1, 2, 5, 4), 0); // 8
    assert_eq!(calc_mod(1, 2, 6, 4), 1); //13
}
