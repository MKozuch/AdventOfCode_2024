use std::fs;

use dayXX::*;

fn main() {
    let input_path = "input/input.txt";
    let input_path = "input/test_input.txt";

    let input = fs::read_to_string(input_path).unwrap();
    
    let data = parse_input(&input);

    let result = do_calculations(&data);

    println!("Result: {result}");
}
