pub const MOD_1000000007: u64 = 1000000007;

pub fn add(module: u64, mut x: u64, mut y: u64) -> u64 {
    x %= module;
    y %= module;
    (x + y) % module
}

pub fn subtract(module: u64, mut x: u64, mut y: u64) -> u64 {
    x %= module;
    y %= module;
    if x >= y {
        x - y
    } else {
        x - y + module
    }
}

pub fn multiply(module: u64, mut x: u64, mut y: u64) -> u64 {
    x %= module;
    y %= module;
    (x * y) % module
}

pub fn factorial(module: u64, n: u64) -> u64 {
    (1..=n).fold(1, |acc, x| multiply(module, acc, x))
}

pub fn power(module: u64, mut x: u64, mut y: u64) -> u64 {
    x %= module;

    let mut res = 1;

    while y > 0 {
        if y & 1 != 0 {
            res = multiply(module, res, x);
        }
        y /= 2;
        x = multiply(module, x, x);
    }

    res
}

pub fn divide(module: u64, x: u64, y: u64) -> u64 {
    multiply(module, x, power(module, y, module - 2))
}

// Visibility: off
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorials() {
        assert_eq!(factorial(MOD_1000000007, 5), 120);
        assert_eq!(factorial(10007, 10), 3628800 % 10007);
    }
}
// Visibility: on
