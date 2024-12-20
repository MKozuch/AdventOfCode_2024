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
