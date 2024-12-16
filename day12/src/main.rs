use std::fs;

use day12::*;

fn main() {
    let input_path = "input/input.txt";
    let input_path = "input/test_input.txt";

    let input = fs::read_to_string(input_path).unwrap();
    
    let data = parse_input(&input);

    let groups = find_groups(&data);

    let result: usize = groups.iter().map(|group| get_fence_cost(&data, group)).sum();
    println!("Result: {result}");
    
    let result: usize = groups.iter().map(|group| get_fence_cost_with_discount(&data, group)).sum();
    println!("Result: {result}");
}
