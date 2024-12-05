use std::fs;
use day04::*;

fn main() {
    let input = fs::read_to_string("input/input.txt").unwrap();
    let arrays = parse_input(&input);

    let total = count_total_xmas(&arrays);
    println!("Total XMAS: {}", total);

    let total = count_x_mas(&input);
    println!("Total X-MAS: {}", total);
}
