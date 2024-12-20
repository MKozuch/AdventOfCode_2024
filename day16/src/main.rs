use std::fs;

use day16::*;

fn main() {
    let input_path = "input/input.txt";
    // let input_path = "input/test_input.txt";
    // let input_path = "input/test_input_2.txt";

    let input = fs::read_to_string(input_path).unwrap();

    let labyrinth = parse_input(&input);

    let path = find_path(&labyrinth).unwrap();

    //draw_labyrinth_and_path(&labyrinth, &path.get_pos_history());
    draw_labyrinth_and_path_2(&labyrinth, &path);
    
    println!("Result: {}", path.cost());
}

//println!("Result: {result}");


// 106476 too low
// 106477 too low

// 107474 wrong
// 107475 wrong 
// 107476 too high