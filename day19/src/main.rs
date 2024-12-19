use std::fs;

use day19::*;

fn main() {
    let input_path = "input/input.txt";
    //let input_path = "input/test_input.txt";

    let input = fs::read_to_string(input_path).unwrap();

    let (input_towels, input_designs) = input.split_once("\n\n").unwrap();

    let towels = parse_towels(input_towels);
    let designs = parse_designs(input_designs);

    // part 1
    {
        let mut valid_designs_count = 0;

        let mut cache = DesignCache::new();
        cache.insert("".to_owned(), Some(vec![]));

        for design in designs.iter() {
            //println!("Testing string {}", design);

            let design_decomp = decompose_design(&design, &towels, &mut cache);
            if design_decomp.is_some() {
                valid_designs_count += 1;
            }
        }

        let result = valid_designs_count;

        println!("Result: {result}");
    }

    // part 2
    {
        let mut cache = DesignCache2::new();
        let towels = TowelSet::from_iter(towels.values().cloned());

        let result_vec: Vec<_> = designs
            .iter()
            .map(|design| count_possible_decomps(design, &towels, &mut cache))
            .collect();

        let result_1 = result_vec.iter().filter(|&&item| item > 0).count();
        println!("Result 1: {result_1}");

        let result_2: usize = result_vec.iter().sum();
        println!("Result 2: {result_2}");
    }
}

// fn has_match(design: &str, towel: &str){
//     design.len() > towel.len() && design.starts_with(pat)
// }
