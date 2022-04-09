#![allow(clippy::many_single_char_names)]

use library::input;

// Visibility: off
// Tests go here
// Visibility: on

pub fn solve(iter: &mut dyn Iterator<Item = &str>) -> String {
    let t: u64 = input::get(iter);
    let mut ans = String::new();

    for _ in 0..t {
        ans += &format!("\n");
    }

    ans
}
