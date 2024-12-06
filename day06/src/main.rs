use std::{cmp::Ordering, fs};

use day06::*;

fn main() {
    //let input_file_path = "input/test_input.txt";
    let input_file_path = "input/input.txt";

    let input = fs::read_to_string(input_file_path).unwrap();

    let init_guard_dir = Direction::Up;

    let (map, init_guard_pos) = parse_input(&input);

    let mut guard_pos = init_guard_pos.clone();
    let mut guard_dir = init_guard_dir.clone();

    let mut position_history = vec![guard_pos];

    loop {
        let next_guard_step = simulate_one_guard_step(&map, &guard_pos, &guard_dir);
        match next_guard_step {
            GuardStepResult::PatrolEnd => break,
            GuardStepResult::NextStep(pos, dir) => {
                guard_pos = pos;
                guard_dir = dir;
                position_history.push(guard_pos);
            }
        }
    }

    println!("Total steps: {}", position_history.len());

    let mut unique_positions = position_history.clone();
    unique_positions.sort_by(|left, right| {
        if left.x() == right.x() && left.y() == right.y() {
            return Ordering::Equal;
        }

        if left.x() == right.x() {
            return left.y().cmp(&right.y());
        }

        return left.x().cmp(&right.x());
    });
    unique_positions.dedup_by(|left, right| left.x() == right.x() && left.y() == right.y());

    println!("Unique positions: {}", unique_positions.len());

    // p2
    let mut possible_obstacle_placements = 0;
    for pos in unique_positions {
        print!("Testing placing obstacle at ({},{})", pos.x(), pos.y());
        let altered_map = map.clone_with_additional_obstacle(&pos);
        let is_looping = is_path_looping(&altered_map, &init_guard_pos, &init_guard_dir);
        if is_looping {
            possible_obstacle_placements += 1;
        }
        println!(" \t result: {is_looping}");
    }

    println!("Possible obstacle placements: {possible_obstacle_placements}");
}
