use std::{fmt::Debug, fs};

use day15::*;

fn main() {
    let input_path = "input/input.txt";
    let input_path = "input/test_input.txt";

    let input = fs::read_to_string(input_path).unwrap();
    
    let (mut warehouse_map, move_list, mut robot) = parse_input(&input);

    for move_dir in move_list {
        simulate_robot_move(&mut warehouse_map, &mut robot, move_dir);
    }

    let result = calc_gps_coords(&warehouse_map);

    draw_warehouse_map(&warehouse_map, Some(robot));
    // let result = do_calculations(&data);

    // println!("Result: {result}");
}
