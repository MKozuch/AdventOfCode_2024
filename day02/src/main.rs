use std::fs;
use day02::*;

fn main() {
    let input = fs::read_to_string("input/input.txt").unwrap();
    let report_list = parse_input(&input);

    let safe_count = report_list.iter().filter(|report| is_safe(report)).count();
    println!("Safe reports count: {safe_count}");

    let safe_with_dampener_count = report_list.iter().filter(|report| is_safe_with_dampener(&report)).count();
    println!("Safe reports with dampener count: {safe_with_dampener_count}");
}
