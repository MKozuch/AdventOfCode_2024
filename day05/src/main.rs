use std::fs;

use day05::*;

fn main() {
    let input =
        fs::read_to_string("input/input.txt").expect("Something went wrong reading the file");

    let (mut rules_input, mut pages_input) = input.split_once("\n\n").unwrap();

    rules_input = rules_input.trim();
    pages_input = pages_input.trim();

    let rules = parse_rules(&rules_input);
    let pages = parse_pages(&pages_input);

    let mut middle_pages = Vec::<i64>::new();
    let mut fixed_middle_pages = Vec::<i64>::new();

    for page in pages {
        if is_order_valid(&page, &rules) {
            middle_pages.push(get_middle_page(&page));
        } else {
            let fixed_page = fix_sorting(&page, &rules);
            fixed_middle_pages.push(get_middle_page(&fixed_page));
        }
    }

    let result = middle_pages.iter().sum::<i64>();
    let result2 = fixed_middle_pages.iter().sum::<i64>();

    println!("Result: {result}");
    println!("Result2: {result2}");
}
