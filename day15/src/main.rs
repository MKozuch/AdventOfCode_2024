use std::{fmt::Debug, fs};

use day15::*;

fn main() {
    let input_path = "input/input.txt";
    //let input_path = "input/test_input.txt";

    let input = fs::read_to_string(input_path).unwrap();
    
    let (mut warehouse_map, move_list, mut robot) = parse_input(&input);

    for (i, move_dir) in move_list.iter().enumerate() {
        simulate_robot_move(&mut warehouse_map, &mut robot, *move_dir);
        // println!("Iteration {}", i);
        // draw_warehouse_map(&warehouse_map, Some(robot));
        // println!()
    }

    let result = calc_gps_coords(&warehouse_map);
    println!("Result: {result}");
}
