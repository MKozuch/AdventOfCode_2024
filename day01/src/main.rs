use std::fs;

use day01::*;

fn main() {
    let input = fs::read_to_string("input/input.txt").unwrap();
    let (mut left, mut right) = parse_input(input);

    left.sort();
    right.sort();

    let dist = calc_dist(&left, &right);
    println!("Dist: {dist}");

    let similarity_score = calc_similarity_score(&left, &right);
    println!("Similariy score: {similarity_score}");
}
