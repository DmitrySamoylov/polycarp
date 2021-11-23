#![allow(dead_code)]

mod check;
mod solve;

fn main() {
    let mut input = String::new();

    std::io::Read::read_to_string(&mut std::io::stdin(), &mut input).unwrap();

    let mut iter = input.split_whitespace().map(AsRef::as_ref);

    let answer = solve::solve(&mut iter);

    print!("{}", answer);
}
