use std::fs;

use day10::*;

fn main() {
    let input_path = "input/input.txt";
    // let input_path = "input/test_input.txt";

    let input = fs::read_to_string(input_path).unwrap();

    let topo_map = parse_input(&input);
    let start_points = find_start_points(&topo_map);

    let mut score_sum = 0;
    let mut rating_sum = 0;

    for start_position in start_points {
        let trails = find_trails_from(&topo_map, &start_position);
        let mut end_points = Vec::<Pos>::from_iter(trails.iter().map(|trail| *trail.last().unwrap()));
        end_points.sort();
        end_points.dedup();

        score_sum += end_points.len();
        rating_sum += trails.len();
    }

    println!("Score sum: {score_sum}");
    println!("Rating sum: {rating_sum}");
}
