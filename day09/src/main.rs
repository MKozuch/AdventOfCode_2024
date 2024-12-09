use std::fs;

use day09::*;

fn main() {
    let input_path = "input/input.txt";
   // let input_path = "input/test_input.txt";

    let input = fs::read_to_string(input_path).unwrap();

    let unrolled = unroll_input(&input);
    let rearranged = rearrange_simple(&unrolled);

    let result = calc_checksum(&rearranged);
    println!("Result: {result}");

    let rearranged_2 = rearrange_smart(&unrolled);
    let result_2 = calc_checksum(&rearranged_2);
    println!("Result 2: {result_2}");
}


