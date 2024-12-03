use std::fs;
use day03::*;

fn main() {
    let input = fs::read_to_string("input/input.txt").unwrap();
    let pairs = parse_input(&input);
    let result = calc_multiplications(&pairs);
    println!("Result: {}", result);

    let pairs_2 = parse_input_2(&input);
    let result_2 = calc_multiplications(&pairs_2);
    println!("Result 2: {}", result_2);
}
