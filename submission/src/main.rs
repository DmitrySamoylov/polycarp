#![allow(dead_code)]

mod check;
mod input;
mod solve;

fn main() {
    let mut input = String::new();

    std::io::Read::read_to_string(&mut std::io::stdin(), &mut input).unwrap();

    let answer = solve::solve(input::Input {
        iter: &mut input.split_whitespace(),
    });

    print!("{}", answer);
}
