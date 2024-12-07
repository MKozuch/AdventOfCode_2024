use std::fs;

use day07::*;

fn main() {
    let input_path = "input/input.txt";
    let input = fs::read_to_string(input_path).unwrap();

    let entries = parse_input(&input);
    let mut result = 0;

    for (test_value, args) in entries.iter() {
        if validate_entry_add_mul(&test_value, &args){
            result += test_value;
        }
    }
    println!("Result: {result}");

    let mut result_2 = 0;
    for (test_value, args) in entries {
        if validate_entry_add_mul_concat(&test_value, &args){
            result_2 += test_value;
        }
    }
    println!("Result 2: {result_2}");
}


