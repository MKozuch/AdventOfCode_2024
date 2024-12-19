use std::fs;

use day18::*;

fn main() {
    let input_path;
    let grid_size;
    let input_len;

    let test_mode = false;

    if test_mode {
        input_path = "input/test_input.txt";
        grid_size = 6_u64 + 1;
        input_len = 12;
    }
    else {
        input_path = "input/input.txt";
        grid_size = 70_u64 + 1;
        input_len = 1024;
    }

    let input = fs::read_to_string(input_path).unwrap();
    let start_point = (0,0);
    let end_point = (grid_size-1 as u64, grid_size-1 as u64);

    let corrupted_blocks: Vec<Pos>= parse_input(&input, 9999);

    // part 1
    let corrupted_blocks_slice = &corrupted_blocks[0..input_len];
    let node = find_path(grid_size, corrupted_blocks_slice, start_point, end_point).unwrap();

    let result = path_cost(&node);

    let path = unroll_path(&node);
    draw_grid(grid_size, corrupted_blocks_slice, &path);

    println!("Result: {result}");
    println!();

    // part2
    let mut known_good = 0;
    let mut known_bad = corrupted_blocks.len()-1;

    loop{   
        let test_point = known_good + (known_bad-known_good)/2;
        let corrupted_blocks_slice = &corrupted_blocks[0..=test_point];

        print!("Checking index: {} between {} and {} ", test_point, known_good, known_bad);
        let node = find_path(grid_size, corrupted_blocks_slice, start_point, end_point);
        println!("\tResult was: {}", node.is_some());

        if node.is_some() {
            known_good = test_point;
        }
        else {
            known_bad = test_point;
        }

        if known_bad == known_good+1{
            println!("i: {}, p: {:?}", known_bad, corrupted_blocks[known_bad]);
            break;
        }
    }
}
